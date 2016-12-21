use ::errors::*;

use futures::*;

use std::sync::Arc;
use std::time::Duration;
use tokio_timer::Timer;

pub trait Runnable {
    fn run(&self) -> ::errors::Result<()>;
}

/// Schedule that the given task should run at a given interval.
pub fn schedule<R>(
    timer: Arc<Timer>,
    interval: Duration,
    runnable: R
) -> Box<Future<Item=(), Error=Error>>
    where R: Runnable + 'static
{
    Box::new(timer.sleep(interval).map_err(|e| e.into()).and_then(move |()| {
        match runnable.run() {
            Ok(()) => {
                schedule(timer, interval, runnable)
            },
            Err(err) => {
                future::err::<(), Error>(err).boxed()
            }
        }
    }))
}

