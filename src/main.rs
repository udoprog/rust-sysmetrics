#![feature(stmt_expr_attributes)]

#[cfg(features = "watch")]
extern crate notify;
extern crate rustc_serialize;
extern crate sysmetrics;
extern crate toml;
extern crate getopts;

#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::result::Result;
use std::env;
use getopts::Options;

use sysmetrics::plugin;
use sysmetrics::logger;

#[derive(Debug)]
enum ConfigError<'a> {
    IO(std::io::Error),
    ParseError(&'a str),
    Plugin(plugin::Error),
    MissingField(&'a str, &'a str),
    MissingPlugin(String)
}

type NewPlugin = fn(toml::Table) -> Result<Box<plugin::Plugin>, plugin::Error>;

fn load_plugin<'a>(path: &'a str, plugins: &HashMap<String, NewPlugin>) -> Result<Box<plugin::Plugin>, ConfigError<'a>> {
    let mut file = try!(fs::File::open(path).map_err(ConfigError::IO));

    let mut content = String::new();
    try!(file.read_to_string(&mut content).map_err(ConfigError::IO));

    let mut parser = toml::Parser::new(&mut content);

    let value = try!(
        parser.parse()
        .ok_or(ConfigError::ParseError(path)));

    let plugin = {
        let plugin_type = try!(
            value.get("type")
            .and_then(toml::Value::as_str)
            .ok_or(ConfigError::MissingField(path, "type")));

        try!(plugins.get(plugin_type).ok_or(ConfigError::MissingPlugin(plugin_type.to_owned())))
    };

    plugin(value).map_err(ConfigError::Plugin)
}

fn print_usage(program: &str, plugins: &HashMap<String, NewPlugin>, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));

    println!("Plugins:");

    for (k, _) in plugins {
        println!("  {}", k);
    }
}

fn load_plugins() -> HashMap<String, NewPlugin> {
    let mut m: HashMap<String, NewPlugin> = HashMap::new();
    m.insert("disk".to_owned(), sysmetrics::plugins::disk::entry);
    m.insert("cpu".to_owned(), sysmetrics::plugins::cpu::entry);
    m.insert("load".to_owned(), sysmetrics::plugins::load::entry);

    #[cfg(feature = "http")]
    m.insert("http_poller".to_owned(), sysmetrics::plugins::http_poller::entry);

    m
}

fn main() {
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help");
    opts.optflag("", "debug", "enable debug logging");
    opts.optmulti("", "confdir", "load configuration files from the given directories", "<dir>");

    #[cfg(feature = "watch")]
    opts.optflag("w", "watch", "enable watching of the configuration directory");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let plugins: HashMap<String, NewPlugin> = load_plugins();

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
        _ => {  }
    }

    let plugin = load_plugin("config/poller.toml", &plugins);

    let confdirs = matches.opt_strs("confdir");

    info!("Directories: {:?}", directories);

    println!("{:?}", plugin);
}
