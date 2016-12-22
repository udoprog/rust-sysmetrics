use ::errors::*;
use ::plugin::*;
use toml;

#[derive(Debug)]
struct LoadInput {
}

impl LoadInput {
    pub fn new() -> LoadInput {
        LoadInput {}
    }
}

impl Input for LoadInput {
    fn setup(&self, _: &PluginFramework) -> Result<Box<InputInstance>> {
        Ok(Box::new(LoadInputInstance::new()))
    }
}

#[derive(Debug)]
struct LoadInputInstance {
}

impl LoadInputInstance {
    pub fn new() -> LoadInputInstance {
        LoadInputInstance {}
    }
}

impl InputInstance for LoadInputInstance {}

pub fn input(_: &PluginKey, _: toml::Value) -> Result<Box<Input>> {
    Ok(Box::new(LoadInput::new()))
}
