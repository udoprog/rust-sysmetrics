use errors::*;
use plugin::*;

#[derive(Debug)]
struct DiskInput {}

impl DiskInput {
    pub fn new() -> DiskInput {
        DiskInput {}
    }
}

impl Input for DiskInput {
    fn setup(&self, _ctx: PluginContext) -> Result<Box<InputInstance>> {
        Ok(Box::new(DiskInputInstance::new()))
    }
}

#[derive(Debug)]
struct DiskInputInstance {}

impl DiskInputInstance {
    pub fn new() -> DiskInputInstance {
        DiskInputInstance {}
    }
}

impl InputInstance for DiskInputInstance {}

pub fn input() -> Result<Box<Input>> {
    Ok(Box::new(DiskInput::new()))
}
