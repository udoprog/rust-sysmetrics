#![feature(box_syntax, box_patterns)]

extern crate sysmon;
extern crate toml;
extern crate getopts;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tokio_signal;
#[macro_use]
extern crate log;
#[cfg(features = "watch")]
extern crate notify;

use sysmon::errors::*;
use sysmon::logger;
use sysmon::parsers::*;
use sysmon::plugin::*;
use sysmon::poller::Poller;
use sysmon::scheduler::*;
use sysmon::updater::Updater;

use futures::*;
use futures::stream::Stream;
use futures_cpupool::CpuPool;
use std::env;
use std::fs;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use tokio_core::reactor::*;

enum LoadedPlugin {
    Input(Box<Input>),
    Output(Box<Output>),
}

fn load_section(plugins: &PluginRegistry, section: toml::Value) -> Result<LoadedPlugin> {
    let plugin_type: String =
        toml::decode(section.clone()).and_then(|value: toml::Table| {
                value.get("type").map(Clone::clone).and_then(toml::decode)
            })
            .ok_or(ErrorKind::TomlDecode)?;

    let plugin_key = parse_plugin_key(plugin_type.as_bytes()).to_full_result()?;

    let ref plugin_type = plugin_key.plugin_type;

    match plugin_key.plugin_kind {
        PluginKind::Input => {
            let entry: &InputEntry = plugins.get_input(plugin_type)
                .ok_or(ErrorKind::MissingPlugin(plugin_key.clone()))?;

            entry(&plugin_key, section.clone()).map(LoadedPlugin::Input)
        }
        PluginKind::Output => {
            let entry: &OutputEntry = plugins.get_output(plugin_type)
                .ok_or(ErrorKind::MissingPlugin(plugin_key.clone()))?;

            entry(&plugin_key, section.clone()).map(LoadedPlugin::Output)
        }
    }
}

fn load_config(path: &String, plugins: &PluginRegistry) -> Result<Vec<LoadedPlugin>> {
    let mut file = fs::File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut parser = toml::Parser::new(&mut content);

    let config = match parser.parse() {
        Some(value) => value,
        None => return Err(ErrorKind::TomlParse(parser.errors).into()),
    };

    let mut instances = Vec::new();

    for (section_key, section) in config.into_iter() {
        instances.push(load_section(plugins, section).chain_err(|| {
            ErrorKind::ConfigSection(section_key)
        })?);
    }

    Ok(instances)
}

fn print_usage(program: &str, plugins: &PluginRegistry, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));

    println!("Plugins:");

    for plugin_type in plugins.input_types() {
        println!("  input/{}", plugin_type);
    }

    for plugin_type in plugins.output_types() {
        println!("  output/{}", plugin_type);
    }
}

fn load_configs(configs: Vec<String>, plugins: &PluginRegistry) -> Result<Vec<LoadedPlugin>> {
    let mut loaded: Vec<LoadedPlugin> = Vec::new();

    for config in configs.iter() {
        info!("loading: {}", config);

        loaded.extend(load_config(config, plugins).chain_err(|| {
            ErrorKind::Config(config.clone())
        })?);
    }

    Ok(loaded)
}

fn setup_plugins(loaded: Vec<LoadedPlugin>,
                 framework: &PluginFramework)
                 -> Result<(Vec<Arc<Box<InputInstance>>>, Arc<Vec<Box<OutputInstance>>>)> {
    let mut input = Vec::new();
    let mut output = Vec::new();

    for l in loaded {
        match l {
            LoadedPlugin::Input(plugin) => {
                input.push(Arc::new(plugin.setup(&framework)?));
            }
            LoadedPlugin::Output(plugin) => {
                output.push(plugin.setup(&framework)?);
            }
        }
    }

    Ok((input, Arc::new(output)))
}

fn setup_opts() -> getopts::Options {
    let mut opts = getopts::Options::new();

    opts.optflag("h", "help", "print this help");
    opts.optflag("", "debug", "enable debug logging");
    opts.optmulti("", "config", "load configuration file", "<file>");

    #[cfg(feature = "watch")]
    opts.optflag("w",
                 "watch",
                 "enable watching of the configuration directory");

    opts
}

/// Configure logging
///
/// If debug (--debug) is specified, logging should be configured with LogLevelFilter::Debug.
fn setup_logger(matches: &getopts::Matches) -> Result<()> {
    let level: log::LogLevelFilter = match matches.opt_present("debug") {
        true => log::LogLevelFilter::Debug,
        false => log::LogLevelFilter::Info,
    };

    logger::init(level)?;

    Ok(())
}

/// Actual main
///
/// Wrapped to allow the returning of chained errors.
fn run() -> Result<()> {
    let plugins: PluginRegistry = sysmon::plugins::load_plugins();

    let args: Vec<String> = env::args().collect();
    let opts = setup_opts();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print_usage(&args[0], &plugins, opts);
            return Err(f.into());
        }
    };

    if matches.opt_present("h") {
        print_usage(&args[0], &plugins, opts);
        return Ok(());
    }

    setup_logger(&matches)?;

    let pool = Arc::new(CpuPool::new(4));
    let framework = PluginFramework { cpupool: pool.clone() };

    let loaded = load_configs(matches.opt_strs("config"), &plugins)?;
    let (input, output) = setup_plugins(loaded, &framework)?;

    let poller = Poller::new(&input, output.clone());
    let updater = Updater::new(&input, pool.clone());

    let update_duration = Duration::new(1, 0);
    let poll_duration = Duration::new(10, 0);

    let mut core = Core::new()?;
    let handle = core.handle();

    let update_interval = Interval::new(update_duration, &handle)?.map_err(Into::into);
    let poll_interval = Interval::new(poll_duration, &handle)?.map_err(Into::into);

    let update = update_interval.and_then(move |_| updater.run());
    let poll = poll_interval.and_then(move |_| poller.run());

    let ctrl_c = core.run(::tokio_signal::ctrl_c(&handle))?;

    let shutdown: BoxFuture<(), Error> = box ctrl_c.map_err(Into::into).for_each(|_| {
        info!("Interrupted");
        Err(ErrorKind::ShutdownError.into())
    });

    let mut futures: Vec<BoxFuture<(), Error>> = Vec::new();
    futures.push(box update.for_each(|_| Ok(())));
    futures.push(box poll.for_each(|_| Ok(())));

    let tasks: BoxFuture<(), Error> = box future::join_all(futures).map(|_| ());
    let combo = future::select_all(vec![tasks, shutdown]);

    info!("Started!");

    match core.run(combo) {
        Ok(_) => {
            println!("Everything is GREAT");
        }
        Err(e) => {
            let (error, _size, _futures) = e;
            return Err(error);
        }
    }

    info!("Shutting down!");

    Ok(())
}

fn main() {
    match run() {
        Err(Error(ErrorKind::ShutdownError, _)) => {
            info!("Shutting down");
        }
        Err(e) => {
            error!("{}", e);

            for e in e.iter().skip(1) {
                error!("  caused by: {}", e);
            }

            if let Some(backtrace) = e.backtrace() {
                error!("  backtrace: {:?}", backtrace);
            }

            ::std::process::exit(1);
        }
        Ok(loaded) => loaded,
    }

    ::std::process::exit(0);
}
