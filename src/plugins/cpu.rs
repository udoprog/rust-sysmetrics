use ::metric::*;
use ::plugin::*;
use ::errors::*;
use ::parsers::*;

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
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {}
    }
}

impl Plugin for Cpu {
    fn setup(&self, framework: &PluginFramework) -> Result<Box<PluginInstance>> {
        let instance = CpuInstance::new(framework.cpupool.clone());

        Ok(Box::new(instance))
    }
}

struct Metrics {
    previous: Option<StatCpu>,
    used_percentage: (Arc<MetricId>, Gauge),
    free_percentage: (Arc<MetricId>, Gauge)
}

impl Metrics {
    pub fn update(&mut self) -> Result<()> {
        let file = File::open("/proc/stat")?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();

        reader.read_line(&mut buffer)?;

        let next = parse_stat_cpu(buffer.as_bytes()).to_full_result()?;

        self.previous = match self.previous {
            None => {
                Some(next)
            }
            Some(ref prev) => {
                let total_diff = next.total() - prev.total();

                if total_diff > 0 {
                    self.used_percentage.1.set(((next.used() - prev.used()) as f64) / total_diff as f64);
                    self.free_percentage.1.set(((next.free() - prev.free()) as f64) / total_diff as f64);
                }

                Some(next)
            }
        };

        Ok(())
    }
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
                previous: None,
                used_percentage: (Arc::new(key("cpu-used-percentage")), Gauge::new()),
                free_percentage: (Arc::new(key("cpu-free-percentage")), Gauge::new())
            })),
            cpupool: cpupool
        }
    }
}

impl PluginInstance for CpuInstance {
    fn poll(&self) -> Result<Samples> {
        let ref mut m = self.metrics.lock()?;

        let results = vec![
            Sample::new(m.free_percentage.0.clone(), m.free_percentage.1.snapshot()),
            Sample::new(m.used_percentage.0.clone(), m.used_percentage.1.snapshot())
        ];

        Ok(results)
    }

    fn update(&self) -> BoxFuture<(), Error> {
        let m = self.metrics.clone();

        self.cpupool.spawn(future::lazy(move || {
            let result: Result<()> = m.lock()
                .map_err(Into::into)
                .and_then(|mut locked| locked.update());

            future::result(result)
        })).boxed()
    }

    fn next_update(&self) -> Duration {
        self.next_update
    }
}

pub fn entry(_: &PluginKey, _: toml::Value) -> Result<Box<Plugin>> {
    Ok(Box::new(Cpu::new()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snapshot() {
    }
}
