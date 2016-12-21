extern crate sysmon;
extern crate toml;
extern crate getopts;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;
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
use futures_cpupool::CpuPool;
use getopts::Options;
use std::env;
use std::fs;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use tokio_timer::Timer;

enum LoadedPlugin {
    Input(Box<Input>),
    Output(Box<Output>),
}

fn load_section(
    plugins: &PluginRegistry,
    section: toml::Value
) -> Result<LoadedPlugin> {
    let plugin_type: String = toml::decode(section.clone())
        .and_then(|value: toml::Table| {
            value.get("type").map(Clone::clone).and_then(toml::decode)
        })
        .ok_or(ErrorKind::TomlDecode)?;

    let plugin_key = parse_plugin_key(plugin_type.as_bytes()).to_full_result()?;

    let ref plugin_type = plugin_key.plugin_type;

    match plugin_key.plugin_kind {
        PluginKind::Input => {
            let entry: &InputEntry = plugins
                .get_input(plugin_type)
                .ok_or(ErrorKind::MissingPlugin(plugin_key.clone()))?;

            entry(&plugin_key, section.clone()).map(LoadedPlugin::Input)
        }
        PluginKind::Output => {
            let entry: &OutputEntry = plugins
                .get_output(plugin_type)
                .ok_or(ErrorKind::MissingPlugin(plugin_key.clone()))?;

            entry(&plugin_key, section.clone()).map(LoadedPlugin::Output)
        }
    }
}

fn load_config(
    path: &String, plugins: &PluginRegistry
) -> Result<Vec<LoadedPlugin>>
{
    let mut file = fs::File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut parser = toml::Parser::new(&mut content);

    let config = match parser.parse() {
        Some(value) => value,
        None => {
            return Err(ErrorKind::TomlParse(parser.errors).into())
        }
    };

    let mut instances = Vec::new();

    for (section_key, section) in config.into_iter() {
        instances.push(load_section(plugins, section).chain_err(|| {
            ErrorKind::ConfigSection(section_key)
        })?);
    }

    Ok(instances)
}

fn print_usage(program: &str, plugins: &PluginRegistry, opts: Options) {
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

fn load_configs(
    configs: &Vec<String>,
    plugins: &PluginRegistry
) -> Result<Vec<LoadedPlugin>>
{
    let mut loaded: Vec<LoadedPlugin> = Vec::new();

    for config in configs {
        info!("loading: {}", config);

        loaded.extend(load_config(config, plugins).chain_err(|| {
            ErrorKind::Config(config.clone())
        })?);
    }

    Ok(loaded)
}

fn run() -> Result<()> {
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help");
    opts.optflag("", "debug", "enable debug logging");
    opts.optmulti("", "config", "load configuration file", "<file>");

    #[cfg(feature = "watch")]
    opts.optflag("w", "watch", "enable watching of the configuration directory");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let plugins: PluginRegistry = sysmon::plugins::load_plugins();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(&program, &plugins, opts);
            return Err(ErrorKind::Message(f.to_string()).into())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, &plugins, opts);
        return Ok(())
    }

    let level: log::LogLevelFilter = match matches.opt_present("debug") {
        true => log::LogLevelFilter::Debug,
        false => log::LogLevelFilter::Info,
    };

    logger::init(level)?;

    let configs = matches.opt_strs("config");

    let loaded = load_configs(&configs, &plugins)?;

    let pool = Rc::new(CpuPool::new(4));

    let timer = Rc::new(Timer::default());

    let framework = PluginFramework {
        cpupool: pool.clone()
    };

    let mut input = Vec::new();
    let mut output = Vec::new();

    for l in loaded {
        match l {
            LoadedPlugin::Input(plugin) => {
                input.push(plugin.setup(&framework)?);
            }
            LoadedPlugin::Output(plugin) => {
                output.push(plugin.setup(&framework)?);
            }
        }
    }

    let poll_duration = Duration::new(0, 10_000_000);
    let update_duration = Duration::new(1, 0);

    let i = Arc::new(input);
    let o = Arc::new(output);
    let poller = Poller::new(i.clone(), o.clone());
    let updater = Updater::new(i.clone());

    let scheduler = Scheduler::new(pool.clone(), timer.clone());

    scheduler.schedule(poll_duration, poller);
    scheduler.schedule(update_duration, updater);

    info!("Started!");

    info!("Shutting down!");

    let _ = polling.join(updating).wait();

    Ok(())
}

fn main() {
    match run() {
        Err(e) => {
            println!("{}", e);

            for e in e.iter().skip(1) {
                println!("  caused by: {}", e);
            }

            if let Some(backtrace) = e.backtrace() {
                println!("  backtrace: {:?}", backtrace);
            }

            ::std::process::exit(1);
        },
        Ok(loaded) => loaded,
    }

    ::std::process::exit(0);
}
