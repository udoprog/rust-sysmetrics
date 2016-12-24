use ::errors::*;
use ::plugin::*;
use ::scheduler::Runnable;
use futures::*;
use std::sync::Arc;

pub struct Poller {
    input: Vec<Arc<Box<InputInstance>>>,
    output: Arc<Vec<Box<OutputInstance>>>,
}

impl Poller {
    pub fn new(input: &Vec<Arc<Box<InputInstance>>>,
               output: Arc<Vec<Box<OutputInstance>>>)
               -> Poller {
        Poller {
            input: input.iter().map(Clone::clone).collect(),
            output: output,
        }
    }
}

impl Runnable for Poller {
    fn run(&self) -> BoxFuture<(), Error> {
        for instance in self.input.iter() {
            let samples = match instance.poll() {
                Err(err) => return future::err(err).boxed(),
                Ok(s) => s,
            };

            for sample in samples {
                for instance in self.output.iter() {
                    instance.feed(&sample);
                }
            }
        }

        future::ok(()).boxed()
    }
}

impl Drop for Poller {
    fn drop(&mut self) {
        info!("Dropping Poller");
    }
}
