use ::errors::*;
use ::plugin::*;
use toml;

#[derive(Deserialize, Debug)]
struct PollerConfig {
    target: Option<String>
}

#[derive(Debug)]
struct HttpPoller {
    config: PollerConfig
}

impl HttpPoller {
    pub fn new(c: PollerConfig) -> HttpPoller {
        return HttpPoller { config: c };
    }
}

impl Plugin for HttpPoller {
    fn setup(&self, _: &PluginFramework) -> Result<Box<PluginInstance>> {
        Ok(Box::new(HttpPollerInstance::new()))
    }
}

#[derive(Debug)]
struct HttpPollerInstance {
}

impl HttpPollerInstance {
    pub fn new() -> HttpPollerInstance {
        return HttpPollerInstance {  };
    }
}

impl PluginInstance for HttpPollerInstance {
}

pub fn entry(_: &PluginKey, config: toml::Value) -> Result<Box<Plugin>> {
    let decoded: Result<PollerConfig> = toml::decode(config).ok_or(ErrorKind::Setup.into());
    let c: PollerConfig = decoded?;

    Ok(Box::new(HttpPoller::new(c)))
}
