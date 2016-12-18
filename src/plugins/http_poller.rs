use ::plugin::*;
use futures::*;
use toml;

#[derive(Deserialize, Debug)]
struct PollerConfig {
    target: Option<String>
}

#[derive(Debug)]
struct HttpPoller {
    key: String,
    config: PollerConfig
}

impl HttpPoller {
    pub fn new(key: String, c: PollerConfig) -> HttpPoller {
        return HttpPoller {
            key: key,
            config: c
        };
    }
}

impl Plugin for HttpPoller {
    fn key(&self) -> &str {
        self.key.as_str()
    }

    fn setup(&self, _: &PluginFramework) -> Box<PluginInstance> {
        Box::new(HttpPollerInstance::new())
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

pub fn entry(key: String, config: toml::Value) -> Result<Box<Plugin>, SetupError> {
    let c: PollerConfig = try!(toml::decode(config).ok_or(SetupError::DecodeError));

    Ok(Box::new(HttpPoller::new(key, c)))
}
