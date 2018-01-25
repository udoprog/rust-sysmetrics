use errors::*;
use plugin::*;
use scheduler::Runnable;
use futures::*;
use std::sync::Arc;

pub struct Poller {
    input: Arc<Vec<Arc<Box<InputInstance>>>>,
    output: Arc<Vec<Box<OutputInstance>>>,
}

impl Poller {
    pub fn new(
        input: Arc<Vec<Arc<Box<InputInstance>>>>,
        output: Arc<Vec<Box<OutputInstance>>>,
    ) -> Poller {
        Poller {
            input: input,
            output: output,
        }
    }
}

impl Runnable for Poller {
    fn run(&self) -> Box<Future<Item = (), Error = Error>> {
        for instance in self.input.iter() {
            let samples = match instance.poll() {
                Err(err) => return Box::new(future::err(err)),
                Ok(s) => s,
            };

            for sample in samples {
                for instance in self.output.iter() {
                    match instance.feed(&sample) {
                        Err(err) => return Box::new(future::err(err)),
                        Ok(()) => (),
                    };
                }
            }
        }

        Box::new(future::ok(()))
    }
}

impl Drop for Poller {
    fn drop(&mut self) {
        info!("Dropping Poller");
    }
}
