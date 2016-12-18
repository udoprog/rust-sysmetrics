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

use futures_cpupool::CpuPool;
use getopts::Options;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;
use std::rc::Rc;
use std::result::Result;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio_timer::Timer;

use sysmon::logger;
use sysmon::plugin;
use sysmon::poller::Poller;
use sysmon::scheduler::*;

#[derive(Debug)]
enum ConfigError {
    IO(std::io::Error),
    ParseError(String),
    PluginSetup(plugin::SetupError),
    BadKey(String),
    MissingPlugin(String)
}

fn load_config(
    path: &String, plugins: &HashMap<String, plugin::Entry>
) -> Result<Vec<Box<plugin::Plugin>>, ConfigError>
{
    let mut file = try!(fs::File::open(path).map_err(ConfigError::IO));

    let mut content = String::new();
    try!(file.read_to_string(&mut content).map_err(ConfigError::IO));

    let mut parser = toml::Parser::new(&mut content);

    let config = try!(
        parser.parse()
            .ok_or(ConfigError::ParseError(path.clone())));

    let mut instances = Vec::new();

    for (key, section) in &config {
        let mut parts = key.split("/");

        let plugin_type: &str = try!(parts.next().ok_or(ConfigError::BadKey(key.to_owned())));
        let entry: &plugin::Entry = try!(plugins.get(plugin_type).ok_or(ConfigError::MissingPlugin(plugin_type.to_owned())));
        let instance: Box<plugin::Plugin> = try!(entry(key.clone(), section.clone()).map_err(ConfigError::PluginSetup));
        instances.push(instance)
    }

    Ok(instances)
}

fn print_usage(program: &str, plugins: &HashMap<String, plugin::Entry>, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));

    println!("Plugins:");

    for (k, _) in plugins {
        println!("  {}", k);
    }
}

fn main() {
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help");
    opts.optflag("", "debug", "enable debug logging");
    opts.optmulti("", "config", "load configuration file", "<file>");

    #[cfg(feature = "watch")]
    opts.optflag("w", "watch", "enable watching of the configuration directory");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let plugins: HashMap<String, plugin::Entry> = sysmon::plugins::load_plugins();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            println!("Failed to parse options: {}", f.to_string());
            println!("");
            print_usage(&program, &plugins, opts);
            return
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, &plugins, opts);
        return;
    }

    let level: log::LogLevelFilter = match matches.opt_present("debug") {
        true => log::LogLevelFilter::Debug,
        false => log::LogLevelFilter::Info,
    };

    match logger::init(level) {
        Err(f) => {
            println!("Failed to initialize log: {}", f.to_string());
            return;
        },
        _ => {}
    }

    let configs = matches.opt_strs("config");
    let mut loaded: Vec<Box<plugin::Plugin>> = Vec::new();

    for config in configs {
        info!("loading: {}", config);
        loaded.extend(load_config(&config, &plugins).unwrap());
    }

    let pool = CpuPool::new(4);

    let timer = Arc::new(Timer::default());

    let framework = plugin::PluginFramework {
        cpupool: Rc::new(pool)
    };

    let instances: Vec<Box<plugin::PluginInstance>> = loaded.into_iter().map(|plugin| {
        plugin.setup(&framework)
    }).collect();

    let poll_duration = Duration::new(5, 0);

    let polling = schedule(timer, poll_duration, Poller::new(instances));

    info!("Started!");

    thread::sleep(Duration::from_millis(1000));

    drop(polling);

    info!("Shutting down!");
}
