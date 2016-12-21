use ::plugin;
use ::scheduler::Runnable;
use ::errors::*;
use std::sync::Arc;
use futures::*;

pub struct Updater {
    instances: Arc<Vec<Box<plugin::InputInstance>>>
}

impl Updater {
    pub fn new(instances: Arc<Vec<Box<plugin::InputInstance>>>) -> Updater {
        Updater {instances: instances}
    }
}

impl Runnable for Updater {
    fn run(&self) -> BoxFuture<(), Error> {
        let mut futures = Vec::new();

        for instance in self.instances.iter() {
            futures.push(instance.update());
        }

        future::join_all(futures).map(|_| ()).boxed()
    }
}

impl Drop for Updater {
    fn drop(&mut self) {
        info!("Goodbye Updater");
    }
}

