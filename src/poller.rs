use ::errors::*;
use ::plugin::*;
use ::scheduler::Runnable;
use futures::*;
use std::sync::Arc;

pub struct Poller {
    input: Arc<Vec<Box<InputInstance>>>,
    output: Arc<Vec<Box<OutputInstance>>>,
}

impl Poller {
    pub fn new(
        input: Arc<Vec<Box<InputInstance>>>,
        output: Arc<Vec<Box<OutputInstance>>>
    ) -> Poller {
        Poller {input: input, output: output}
    }
}

impl Runnable for Poller {
    fn run(&self) -> BoxFuture<(), Error> {
        let mut samples: Vec<Sample> = Vec::new();

        for instance in self.input.iter() {
            match instance.poll() {
                Err(err) => return future::err(err).boxed(),
                Ok(s) => samples.extend(s),
            }
        }

        for sample in samples {
            for instance in self.output.iter() {
                instance.feed(&sample);
            }
        }

        future::ok(()).boxed()
    }
}

impl Drop for Poller {
    fn drop(&mut self) {
        info!("Goodbye Poller");
    }
}

