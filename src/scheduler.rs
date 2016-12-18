use ::errors::MainError;

use futures::*;

use std::sync::Arc;
use std::time::Duration;
use tokio_timer::Timer;

pub trait Runnable {
    fn run(&self) -> Result<(), MainError>;
}

/// Schedule that the given task should run at a given interval.
pub fn schedule<R>(
    timer: Arc<Timer>,
    interval: Duration,
    runnable: R
) -> Box<Future<Item=(), Error=MainError>>
    where R: Runnable + 'static
{
    Box::new(timer.sleep(interval).map_err(MainError::Timer).and_then(move |()| {
        match runnable.run() {
            Ok(()) => {
                schedule(timer, interval, runnable)
            },
            Err(err) => {
                future::err::<(), MainError>(err).boxed()
            }
        }
    }))
}

