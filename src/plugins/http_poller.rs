use ::errors::*;
use ::plugin::*;
use toml;

#[derive(Deserialize, Debug)]
struct HttpInputConfig {
    target: String,
}

#[derive(Debug)]
struct HttpInput {
    config: HttpInputConfig,
}

impl HttpInput {
    pub fn new(c: HttpInputConfig) -> HttpInput {
        return HttpInput { config: c };
    }
}

impl Input for HttpInput {
    fn setup(&self, _: &PluginFramework) -> Result<Box<InputInstance>> {
        Ok(Box::new(HttpInputInstance::new()))
    }
}

#[derive(Debug)]
struct HttpInputInstance {
}

impl HttpInputInstance {
    pub fn new() -> HttpInputInstance {
        return HttpInputInstance {};
    }
}

impl InputInstance for HttpInputInstance {}

pub fn input(config: &toml::Table) -> Result<Box<Input>> {
    let c: HttpInputConfig =
        toml::decode(toml::Value::Table(config.clone())).ok_or(ErrorKind::Setup)?;
    Ok(Box::new(HttpInput::new(c)))
}
