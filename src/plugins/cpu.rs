use toml;

use ::metric;
use ::plugin;

#[derive(Debug)]
struct CPU {
    user: metric::gauge::Gauge,
    total: metric::gauge::Gauge
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            user: metric::gauge::Gauge::new(),
            total: metric::gauge::Gauge::new()
        }
    }
}

impl plugin::Plugin for CPU {
}

pub fn entry(_: toml::Table) -> Result<Box<plugin::Plugin>, plugin::Error> {
    Ok(Box::new(CPU::new()))
}
