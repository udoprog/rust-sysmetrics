#![feature(proc_macro)]

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
extern crate serde;

use sysmon::config::*;
use sysmon::errors::*;
use sysmon::logger;
use sysmon::plugin::*;
use sysmon::poller::Poller;
use sysmon::scheduler::*;
use sysmon::updater::Updater;

use futures::*;
use futures::stream::Stream;
use futures_cpupool::CpuPool;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::sync::Arc;
use tokio_core::reactor::*;

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

fn load_configs(paths: Vec<String>) -> Result<(Config, Vec<Box<PluginSetup>>)> {
    let mut setups = Vec::new();

    let mut config = Config::new();

    for path in paths.iter() {
        info!("loading: {}", path);
        setups.push(load_config(&mut config, path)?);
    }

    Ok((config, setups))
}

fn setup_plugins(
    setups: Vec<Box<PluginSetup>>,
    config: &Config,
    plugins: &PluginRegistry,
    partial_context: &PartialPluginContext
) -> Result<(Arc<Vec<Arc<Box<InputInstance>>>>, Arc<Vec<Box<OutputInstance>>>)>
{
    let mut inputs: Vec<Arc<Box<InputInstance>>> = Vec::new();
    let mut outputs: Vec<Box<OutputInstance>> = Vec::new();

    for setup in setups {
        let (input, output) = setup(&config, plugins, partial_context)?;

        inputs.extend(input);
        outputs.extend(output);
    }

    Ok((Arc::new(inputs), Arc::new(outputs)))
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

    let (config, setups) = load_configs(matches.opt_strs("config"))?;

    let cpupool = Arc::new(CpuPool::new(config.threads()));
    let core = Rc::new(RefCell::new(Core::new()?));

    let (input, output) = {
        let partial_context = PartialPluginContext::new(cpupool.clone(), core.clone());
        setup_plugins(setups, &config, &plugins, &partial_context)?
    };

    let poller = Poller::new(input.clone(), output.clone());
    let updater = Updater::new(input.clone(), cpupool.clone());

    let ref mut core = core.try_borrow_mut()?;

    let handle = core.handle();

    let update_interval = Interval::new(config.update_interval, &handle)?.map_err(Into::into);
    let poll_interval = Interval::new(config.poll_interval, &handle)?.map_err(Into::into);

    let update = update_interval.and_then(move |_| updater.run());
    let poll = poll_interval.and_then(move |_| poller.run());

    let ctrl_c = core.run(::tokio_signal::ctrl_c(&handle))?;

    let shutdown: BoxFuture<(), Error> = ctrl_c.map_err(Into::into).for_each(|_| {
        info!("Interrupted");
        Err(ErrorKind::Shutdown.into())
    }).boxed();

    let mut futures: Vec<BoxFuture<(), Error>> = Vec::new();
    futures.push(update.for_each(|_| Ok(())).boxed());
    futures.push(poll.for_each(|_| Ok(())).boxed());

    let tasks: BoxFuture<(), Error> = future::join_all(futures).map(|_| ()).boxed();
    let combo = future::select_all(vec![tasks, shutdown]);

    info!("Started!");

    match core.run(combo) {
        Err((Error(ErrorKind::Shutdown, ..), ..)) => {
        }
        Ok(..) => {
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
        _ => {}
    };

    ::std::process::exit(0);
}
