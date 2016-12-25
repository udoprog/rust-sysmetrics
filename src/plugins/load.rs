use ::errors::*;
use ::plugin::*;

#[derive(Debug)]
struct LoadInput {
}

impl Input for LoadInput {
    fn setup(&self, _ctx: PluginContext) -> Result<Box<InputInstance>> {
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

pub fn input() -> Result<Box<Input>> {
    Ok(Box::new(LoadInput {}))
}
