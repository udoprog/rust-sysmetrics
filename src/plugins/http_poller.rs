use toml;

use ::plugin;

#[derive(Debug, RustcDecodable)]
struct PollerConfig {
    target: Option<String>
}

#[derive(Debug)]
struct Poller {
    config: PollerConfig
}

impl Poller {
    pub fn new(c: PollerConfig) -> Poller {
        return Poller { config: c };
    }
}

impl plugin::Plugin for Poller {
}

pub fn entry(value: toml::Table) -> Result<Box<plugin::Plugin>, plugin::Error> {
    let c: PollerConfig = try!(toml::decode(toml::Value::Table(value))
        .ok_or(plugin::Error::DecodeError));

    Ok(Box::new(Poller::new(c)))
}
