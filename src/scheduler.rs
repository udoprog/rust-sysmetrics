use ::errors::*;

use futures::*;

use std::sync::Arc;
use std::time::Duration;
use tokio_timer::Timer;

pub trait Runnable {
    fn run(&self) -> BoxFuture<(), Error>;
}

/// Schedule that the given task should run at a given interval.
pub fn schedule<R>(
    timer: Arc<Timer>,
    interval: Duration,
    runnable: R
) -> Box<Future<Item=(), Error=Error>>
    where R: Runnable + 'static
{
    Box::new(timer.sleep(interval).map_err(Into::into).and_then(move |()| {
        runnable.run().and_then(move |()| {
            schedule(timer, interval, runnable)
        })
    }))
}

