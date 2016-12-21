use ::plugin;
use ::scheduler::Runnable;
use ::errors::*;
use std::sync::Arc;
use futures::*;

pub struct Updater {
    instances: Arc<Vec<Box<plugin::PluginInstance>>>
}

impl Updater {
    pub fn new(instances: Arc<Vec<Box<plugin::PluginInstance>>>) -> Updater {
        Updater {instances: instances}
    }
}

impl Runnable for Updater {
    fn run(&self) -> BoxFuture<(), Error> {
        let mut futures = Vec::new();

        for instance in self.instances.iter() {
            futures.push(instance.update());
        }

        let all = future::join_all(futures).map(|res| ());

        all.boxed()
    }
}

impl Drop for Updater {
    fn drop(&mut self) {
        info!("Goodbye Updater");
    }
}

