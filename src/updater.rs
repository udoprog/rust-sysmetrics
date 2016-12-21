use ::errors::*;
use ::plugin::*;
use ::scheduler::Runnable;
use futures::*;
use std::sync::Arc;

pub struct Updater {
    instances: Arc<Vec<Box<InputInstance>>>
}

impl Updater {
    pub fn new(instances: Arc<Vec<Box<InputInstance>>>) -> Updater {
        Updater {instances: instances}
    }
}

impl Runnable for Updater {
    fn run(&self) -> BoxFuture<(), Error> {
        let futures: Vec<_> = self.instances.iter().map(|b| b.update()).collect();
        future::join_all(futures).map(|_| ()).boxed()
    }
}

impl Drop for Updater {
    fn drop(&mut self) {
        info!("Goodbye Updater");
    }
}

