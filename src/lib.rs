#![feature(integer_atomics)]
#![feature(conservative_impl_trait)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

extern crate getopts;
extern crate histogram;
extern crate num;
extern crate time;
extern crate toml;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_io;
extern crate tokio_timer;
extern crate tokio_core;
extern crate num_cpus;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod parsers;
pub mod plugins;
pub mod plugin;
pub mod metric;
pub mod scheduler;
pub mod errors;
pub mod poller;
pub mod updater;
pub mod config;
