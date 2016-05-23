use toml;

use ::plugin;

#[derive(Debug)]
struct DiskPoller {
}

impl DiskPoller {
    pub fn new() -> DiskPoller {
        DiskPoller {}
    }
}

impl plugin::Plugin for DiskPoller {
}

pub fn entry(_: toml::Table) -> Result<Box<plugin::Plugin>, plugin::Error> {
    Ok(Box::new(DiskPoller::new()))
}
