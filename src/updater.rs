use errors::*;
use futures::*;
use plugin::InputInstance;
use scheduler::Runnable;
use futures_cpupool::CpuPool;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct InputInstanceState {
    /// Only permit one update at a time.
    in_progress: Arc<AtomicBool>,
    instance: Arc<Box<InputInstance>>,
}

pub struct Updater {
    states: Vec<InputInstanceState>,
    pool: Arc<CpuPool>,
}

impl Updater {
    pub fn new(input: Arc<Vec<Arc<Box<InputInstance>>>>, pool: Arc<CpuPool>) -> Updater {
        let states: Vec<_> = input
            .iter()
            .map(|i| {
                InputInstanceState {
                    in_progress: Arc::new(AtomicBool::new(false)),
                    instance: i.clone(),
                }
            })
            .collect();

        Updater {
            states: states,
            pool: pool,
        }
    }
}

impl Runnable for Updater {
    fn run(&self) -> Box<Future<Item = (), Error = Error>> {
        let futures: Vec<_> = self.states
            .iter()
            .map(|state| {
                let in_progress = state.in_progress.clone();
                let should_update =
                    in_progress.compare_and_swap(false, true, Ordering::Relaxed) == false;

                match should_update {
                    true => {
                        Box::new(self.pool.spawn(state.instance.update().map(move |_| {
                            in_progress.store(false, Ordering::Relaxed);
                            ()
                        }))) as Box<Future<Item = (), Error = Error>>
                    }
                    false => {
                        info!("Update already in progress for: {:?}", state.instance);

                        Box::new(future::ok(()))
                    }
                }
            })
            .collect();

        Box::new(future::join_all(futures).map(|_| ()))
    }
}

impl Drop for Updater {
    fn drop(&mut self) {
        info!("Dropping Updater");
    }
}
