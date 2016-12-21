use ::metric::*;
use ::plugin::*;
use ::errors::*;

use futures::*;
use futures_cpupool::*;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use toml;

#[derive(Debug)]
struct Cpu {
    key: String
}

impl Cpu {
    pub fn new(key: String) -> Cpu {
        Cpu { key: key.to_owned() }
    }
}

impl Plugin for Cpu {
    fn key(&self) -> &str {
        self.key.as_str()
    }

    fn setup(&self, framework: &PluginFramework) -> Result<Box<PluginInstance>> {
        let instance = CpuInstance::new(framework.cpupool.clone());

        Ok(Box::new(instance))
    }
}

struct Metrics {
    used_percentage_id: Arc<MetricId>,
    used_percentage: Gauge,
    free_percentage_id: Arc<MetricId>,
    free_percentage: Gauge,
}

struct CpuInstance {
    metrics: Arc<Mutex<Metrics>>,
    cpupool: Rc<CpuPool>,
    next_update: Duration
}

impl fmt::Debug for CpuInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CpuInstance")
    }
}

impl CpuInstance {
    pub fn new(cpupool: Rc<CpuPool>) -> CpuInstance {
        CpuInstance {
            next_update: Duration::from_millis(1000),
            metrics: Arc::new(Mutex::new(Metrics {
                used_percentage_id: Arc::new(key("cpu-used-percentage")),
                used_percentage: Gauge::new(),
                free_percentage_id: Arc::new(key("cpu-free-percentage")),
                free_percentage: Gauge::new(),
            })),
            cpupool: cpupool
        }
    }

    fn update_inner(&mut self) -> Box<Fn() -> Result<()> + Send> {
        let metrics = self.metrics.clone();

        return Box::new(move || {
            let file = File::open("/proc/stat")?;
            let mut reader = BufReader::new(file);
            let mut buffer = String::new();

            reader.read_line(&mut buffer)?;

            info!("TODO(PARSE): {:?}", buffer);

            let mut metrics = metrics.lock()?;

            metrics.used_percentage.set(42.0 as f64);
            Ok(())
        });
    }
}

impl PluginInstance for CpuInstance {
    fn poll(&self) -> Result<Samples> {
        let ref mut m = self.metrics.lock()?;

        let results = vec![
            Sample::new(m.free_percentage_id.clone(), m.free_percentage.snapshot()),
            Sample::new(m.used_percentage_id.clone(), m.used_percentage.snapshot())
        ];

        Ok(results)
    }

    fn update(&mut self) -> BoxFuture<(), Error> {
        let op = self.update_inner();

        self.cpupool.spawn(future::lazy(move || future::result(op()))).boxed()
    }

    fn next_update(&self) -> Duration {
        self.next_update
    }
}

pub fn entry(key: String, _: toml::Value) -> Result<Box<Plugin>> {
    Ok(Box::new(Cpu::new(key)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snapshot() {
    }
}
