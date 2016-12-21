use ::errors::*;

use futures::*;

use std::rc::Rc;
use std::time::Duration;
use tokio_timer::Timer;
use futures_cpupool::CpuPool;

pub trait Runnable {
    fn run(&self) -> BoxFuture<(), Error>;
}

pub struct Scheduler {
    pool: Rc<CpuPool>,
    timer: Rc<Timer>,
}

impl Scheduler {
    pub fn new(pool: Rc<CpuPool>, timer: Rc<Timer>) -> Scheduler {
        Scheduler {
            pool: pool,
            timer: timer,
        }
    }

    pub fn schedule<R>(&self, duration: Duration, runnable: R)
        where R: Runnable + 'static
    {
        self.timer.sleep(duration).and_then(move |_| {
            runnable.run().and_then(move |_| {
                self.schedule(duration, runnable);
                future::ok(());
            })
        });
    }
}
