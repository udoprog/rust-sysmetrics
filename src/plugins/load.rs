use ::plugin::*;
use futures::*;
use toml;

#[derive(Debug)]
struct LoadPoller {
    key: String
}

impl LoadPoller {
    pub fn new(key: String) -> LoadPoller {
        LoadPoller {
            key: key
        }
    }
}

impl Plugin for LoadPoller {
    fn key(&self) -> &str {
        self.key.as_str()
    }

    fn setup(&self, _: &PluginFramework) -> Box<PluginInstance> {
        Box::new(LoadInstance::new())
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

pub fn entry(key: String, _: toml::Value) -> Result<Box<Plugin>, SetupError> {
    Ok(Box::new(LoadPoller::new(key)))
}
