use ::plugin;
use ::scheduler::Runnable;
use ::errors::*;
use std::sync::Arc;

pub struct Updater {
    instances: Arc<Vec<Box<plugin::PluginInstance>>>
}

impl Updater {
    pub fn new(instances: Arc<Vec<Box<plugin::PluginInstance>>>) -> Updater {
        Updater {instances: instances}
    }
}

impl Runnable for Updater {
    fn run(&self) -> Result<()> {
        for instance in self.instances.iter() {
            // instance.update().wait();
        }

        Ok(())
    }
}

impl Drop for Updater {
    fn drop(&mut self) {
        info!("Goodbye Updater");
    }
}

