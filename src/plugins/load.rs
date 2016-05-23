use toml;

use ::plugin;

#[derive(Debug)]
struct LoadPoller {
}

impl LoadPoller {
    pub fn new() -> LoadPoller {
        LoadPoller {}
    }
}

impl plugin::Plugin for LoadPoller {
}

pub fn entry(_: toml::Table) -> Result<Box<plugin::Plugin>, plugin::Error> {
    Ok(Box::new(LoadPoller::new()))
}
