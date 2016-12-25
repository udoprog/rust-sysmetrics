use ::errors::*;
use ::plugin::*;
use toml;

#[derive(Debug)]
struct DiskInput {
}

impl DiskInput {
    pub fn new() -> DiskInput {
        DiskInput {}
    }
}

impl Input for DiskInput {
    fn setup(&self, _: &PluginFramework) -> Result<Box<InputInstance>> {
        Ok(Box::new(DiskInputInstance::new()))
    }
}

#[derive(Debug)]
struct DiskInputInstance {
}

impl DiskInputInstance {
    pub fn new() -> DiskInputInstance {
        DiskInputInstance {}
    }
}

impl InputInstance for DiskInputInstance {}

pub fn input(_id: &str, _config: &toml::Table) -> Result<Box<Input>> {
    Ok(Box::new(DiskInput::new()))
}
