use ::errors::*;
use ::plugin::*;
use toml;

#[derive(Debug)]
struct LoadPoller {
}

impl LoadPoller {
    pub fn new() -> LoadPoller {
        LoadPoller {}
    }
}

impl Plugin for LoadPoller {
    fn setup(&self, _: &PluginFramework) -> Result<Box<PluginInstance>> {
        Ok(Box::new(LoadInstance::new()))
    }
}

#[derive(Debug)]
struct LoadInstance {
}

impl LoadInstance {
    pub fn new() -> LoadInstance {
        LoadInstance {  }
    }
}

impl PluginInstance for LoadInstance {
}

pub fn entry(_: &PluginKey, _: toml::Value) -> Result<Box<Plugin>> {
    Ok(Box::new(LoadPoller::new()))
}
