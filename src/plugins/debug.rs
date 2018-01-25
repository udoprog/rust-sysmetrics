use errors::*;
use plugin::*;

#[derive(Debug)]
struct DebugOutput {}

impl Output for DebugOutput {
    fn setup(&self, ctx: PluginContext) -> Result<Box<OutputInstance>> {
        Ok(Box::new(DebugOutputInstance::new(ctx.id.clone())))
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
    fn feed(&self, sample: &Sample) -> Result<()> {
        info!("  debug: {:?} {:?}", self.id, sample.metric_id);
        info!("      => {}", sample.value);

        Ok(())
    }
}

pub fn output() -> Result<Box<Output>> {
    Ok(Box::new(DebugOutput {}))
}
