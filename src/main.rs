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

use futures::*;
use futures_cpupool::CpuPool;
use getopts::Options;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio_timer::Timer;

use sysmon::logger;
use sysmon::plugin;
use sysmon::poller::Poller;
use sysmon::updater::Updater;
use sysmon::scheduler::*;
use sysmon::errors::*;

type PluginRegistry = HashMap<String, plugin::Entry>;

fn load_instance(
    plugins: &PluginRegistry, key: &String, section: &toml::Value
) -> Result<Box<plugin::Plugin>> {
    let mut parts = key.split("/");

    let plugin_type: &str = parts
        .next()
        .ok_or(ErrorKind::ConfigKey(key.to_owned()))?;

    let entry: &plugin::Entry = plugins
        .get(plugin_type)
        .ok_or(ErrorKind::MissingPlugin(plugin_type.to_owned()))?;

    entry(key.clone(), section.clone())
}

fn load_config(
    path: &String, plugins: &PluginRegistry
) -> Result<Vec<Box<plugin::Plugin>>>
{
    let mut file = fs::File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut parser = toml::Parser::new(&mut content);

    let config = try!(
        parser.parse()
            .ok_or(ErrorKind::ConfigParse(path.clone())));

    let mut instances = Vec::new();

    for (key, section) in &config {
        let instance = load_instance(plugins, key, section)
            .chain_err(|| ErrorKind::ConfigParse(path.clone()))?;
        instances.push(instance)
    }

    Ok(instances)
}

fn print_usage(program: &str, plugins: &PluginRegistry, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));

    println!("Plugins:");

    for (k, _) in plugins {
        println!("  {}", k);
    }
}

fn load_configs(
    configs: &Vec<String>,
    plugins: &PluginRegistry
) -> Result<Vec<Box<plugin::Plugin>>>
{
    let mut loaded: Vec<Box<plugin::Plugin>> = Vec::new();

    for config in configs {
        info!("loading: {}", config);
        loaded.extend(load_config(config, plugins)?);
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

    let pool = CpuPool::new(4);

    let timer = Arc::new(Timer::default());

    let framework = plugin::PluginFramework {
        cpupool: Rc::new(pool)
    };

    let mut instances = Vec::new();

    for plugin in loaded {
        instances.push(plugin.setup(&framework)?);
    }

    let poll_duration = Duration::new(1, 0);
    let update_duration = Duration::new(1, 0);

    let borrowed = Arc::new(instances);
    let polling = schedule(timer.clone(), poll_duration, Poller::new(borrowed.clone()));
    let updating = schedule(timer.clone(), update_duration, Updater::new(borrowed.clone()));

    info!("Started!");

    // thread::sleep(Duration::from_millis(10000));

    info!("Shutting down!");

    future::join_all(vec![polling, updating]).wait();

    Ok(())
}

fn main() {
    match run() {
        Err(e) => {
            println!("error: {}", e);

            for e in e.iter().skip(1) {
                println!("caused by: {}", e);
            }

            if let Some(backtrace) = e.backtrace() {
                println!("backtrace: {:?}", backtrace);
            }

            ::std::process::exit(1);
        },
        Ok(loaded) => loaded,
    }

    ::std::process::exit(0);
}
