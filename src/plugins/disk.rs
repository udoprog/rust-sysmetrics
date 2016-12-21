use ::errors::*;
use ::plugin::*;
use toml;

#[derive(Debug)]
struct DiskPoller {
}

impl DiskPoller {
    pub fn new() -> DiskPoller {
        DiskPoller { }
    }
}

impl Plugin for DiskPoller {
    fn setup(&self, _: &PluginFramework) -> Result<Box<PluginInstance>> {
        Ok(Box::new(DiskInstance::new()))
    }
}

#[derive(Debug)]
struct DiskInstance {
}

impl DiskInstance {
    pub fn new() -> DiskInstance {
        DiskInstance {}
    }
}

impl PluginInstance for DiskInstance {
}

pub fn entry(_: &PluginKey, _: toml::Value) -> Result<Box<Plugin>> {
    Ok(Box::new(DiskPoller::new()))
}
