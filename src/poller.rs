use ::plugin;
use ::scheduler::Runnable;
use ::errors::*;
use std::sync::Arc;
use futures::*;

pub struct Poller {
    instances: Arc<Vec<Box<plugin::PluginInstance>>>
}

impl Poller {
    pub fn new(instances: Arc<Vec<Box<plugin::PluginInstance>>>) -> Poller {
        Poller {instances: instances}
    }
}

impl Runnable for Poller {
    fn run(&self) -> BoxFuture<(), Error> {
        info!("Polling for data...");

        let mut samples: Vec<plugin::Sample> = Vec::new();

        for instance in self.instances.iter() {
            match instance.poll() {
                Err(err) => return future::err(err).boxed(),
                Ok(s) => samples.extend(s),
            }
        }

        for sample in samples {
            info!("Sample: {:?}", sample);
        }

        future::ok(()).boxed()
    }
}

impl Drop for Poller {
    fn drop(&mut self) {
        info!("Goodbye Poller");
    }
}

