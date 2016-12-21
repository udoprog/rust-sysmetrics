use ::errors::*;
use ::plugin::*;
use toml;

#[derive(Debug)]
struct DebugOutput {
}

impl DebugOutput {
    pub fn new() -> DebugOutput {
        DebugOutput {}
    }
}

impl Output for DebugOutput {
    fn setup(&self, _: &PluginFramework) -> Result<Box<OutputInstance>> {
        Ok(Box::new(DebugOutputInstance::new()))
    }
}

#[derive(Debug)]
struct DebugOutputInstance {
}

impl DebugOutputInstance {
    pub fn new() -> DebugOutputInstance {
        DebugOutputInstance {  }
    }
}

impl OutputInstance for DebugOutputInstance {
    fn feed(&self, sample: &Sample) {
        info!("  debug: {:?}", sample);
    }
}

pub fn output(_: &PluginKey, _: toml::Value) -> Result<Box<Output>> {
    Ok(Box::new(DebugOutput::new()))
}
