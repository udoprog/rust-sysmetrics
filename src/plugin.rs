use std::fmt::Debug;

use metric;

pub trait Plugin: Debug  {
    fn register(&self) -> Vec<metric::Metric> {
        Vec::new()
    }
}

#[derive(Debug)]
pub enum Error {
    DecodeError
}
