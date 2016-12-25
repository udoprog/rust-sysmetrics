use ::errors::*;
use ::plugin::*;
use toml;

#[derive(Debug)]
struct DebugOutput {
    id: String,
}

impl DebugOutput {
    pub fn new(id: String) -> DebugOutput {
        DebugOutput { id: id }
    }
}

impl Output for DebugOutput {
    fn setup(&self, _: &PluginFramework) -> Result<Box<OutputInstance>> {
        Ok(Box::new(DebugOutputInstance::new(self.id.clone())))
    }
}

#[derive(Debug)]
struct DebugOutputInstance {
    id: String,
}

impl DebugOutputInstance {
    pub fn new(id: String) -> DebugOutputInstance {
        DebugOutputInstance { id: id }
    }
}

impl OutputInstance for DebugOutputInstance {
    fn feed(&self, sample: &Sample) {
        info!("  debug: {:?} {:?}", self.id, sample.metric_id);
        info!("      => {}", sample.value);
    }
}

pub fn output(id: &str, _config: &toml::Table) -> Result<Box<Output>> {
    Ok(Box::new(DebugOutput::new(id.to_owned())))
}
