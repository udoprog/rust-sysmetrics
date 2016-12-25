//! Snoop plugin that exposes metrics on a local socket.

use ::errors::*;
use ::plugin::*;

#[derive(Deserialize, Debug)]
struct SnoopInputConfig {
    target: String,
}

#[derive(Debug)]
struct SnoopOutput {
}

impl Output for SnoopOutput {
    fn setup(&self, ctx: PluginContext) -> Result<Box<OutputInstance>> {
        Ok(Box::new(SnoopOutputInstance::new(ctx.id.clone())))
    }
}

#[derive(Debug)]
struct SnoopOutputInstance {
    id: String,
}

impl SnoopOutputInstance {
    pub fn new(id: String) -> SnoopOutputInstance {
        SnoopOutputInstance { id: id }
    }
}

impl OutputInstance for SnoopOutputInstance {
    fn feed(&self, sample: &Sample) {
        info!("  debug: {:?} {:?}", self.id, sample.metric_id);
        info!("      => {}", sample.value);
    }
}

pub fn output() -> Result<Box<Output>> {
    Ok(Box::new(SnoopOutput {}))
}
