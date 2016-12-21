#![feature(proc_macro)]
#![feature(integer_atomics)]
// impl traits are awesome (for iterators)
#![feature(conservative_impl_trait)]
// error-chain macros recurse _a lot_
#![recursion_limit = "1024"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

extern crate histogram;
extern crate num;
extern crate time;
extern crate toml;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;

#[macro_use]
extern crate serde_derive;

pub mod parsers;
pub mod plugins;
pub mod plugin;
pub mod metric;
pub mod logger;
pub mod scheduler;
pub mod errors;
pub mod poller;
pub mod updater;
