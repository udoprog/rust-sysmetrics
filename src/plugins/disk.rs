use ::errors::*;
use ::plugin::*;
use futures::*;
use toml;

#[derive(Debug)]
struct DiskPoller {
    key: String
}

impl DiskPoller {
    pub fn new(key: String) -> DiskPoller {
        DiskPoller {
            key: key
        }
    }
}

impl Plugin for DiskPoller {
    fn key(&self) -> &str {
        self.key.as_str()
    }

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

pub fn entry(key: String, _: toml::Value) -> Result<Box<Plugin>> {
    Ok(Box::new(DiskPoller::new(key)))
}
