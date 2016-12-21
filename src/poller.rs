use ::plugin;
use ::scheduler::Runnable;
use ::errors::*;
use std::sync::Arc;

pub struct Poller {
    instances: Arc<Vec<Box<plugin::PluginInstance>>>
}

impl Poller {
    pub fn new(instances: Arc<Vec<Box<plugin::PluginInstance>>>) -> Poller {
        Poller {instances: instances}
    }
}

impl Runnable for Poller {
    fn run(&self) -> Result<()> {
        info!("Polling for data...");

        let mut samples: Vec<plugin::Sample> = Vec::new();

        for instance in self.instances.iter() {
            samples.extend(try!(instance.poll()));
        }

        for sample in samples {
            info!("Sample: {:?}", sample);
        }

        Ok(())
    }
}

impl Drop for Poller {
    fn drop(&mut self) {
        info!("Goodbye Poller");
    }
}

