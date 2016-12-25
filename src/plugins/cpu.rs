use ::metric::*;
use ::plugin::*;
use ::errors::*;
use ::parsers::*;

use futures::*;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead};
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

impl Input for Cpu {
    fn setup(&self, _framework: &PluginFramework) -> Result<Box<InputInstance>> {
        let instance = CpuInputInstance::new();

        Ok(Box::new(instance))
    }
}

struct Metrics {
    previous: Option<StatCpu>,
    used: (Arc<MetricId>, Gauge),
    free: (Arc<MetricId>, Gauge),
}

impl Metrics {
    pub fn update(&mut self) -> Result<()> {
        let file = File::open("/proc/stat")?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();

        reader.read_line(&mut buffer)?;

        let next = parse_stat_cpu(buffer.as_bytes()).to_full_result()?;

        self.previous = match self.previous {
            None => Some(next),
            Some(ref prev) => {
                let total_diff = next.total() - prev.total();

                if total_diff > 0 {
                    let differ = |n, p| ((n - p) as f64) / total_diff as f64;

                    self.used.1.set(differ(next.used(), prev.used()));
                    self.free.1.set(differ(next.free(), prev.free()));
                }

                Some(next)
            }
        };

        Ok(())
    }
}

struct CpuInputInstance {
    metrics: Arc<Mutex<Metrics>>,
    next_update: Duration,
}

impl fmt::Debug for CpuInputInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CpuInputInstance")
    }
}

impl CpuInputInstance {
    pub fn new() -> CpuInputInstance {
        let system = key("system");

        CpuInputInstance {
            next_update: Duration::from_millis(1000),
            metrics: Arc::new(Mutex::new(Metrics {
                previous: None,
                used: (Arc::new(system.tags(&[("what", "cpu-used"), ("unit", "%")])
                           .meaning(&["what"])),
                       Gauge::new()),
                free: (Arc::new(system.tags(&[("what", "cpu-free"), ("unit", "%")])
                           .meaning(&["what"])),
                       Gauge::new()),
            })),
        }
    }
}

impl InputInstance for CpuInputInstance {
    fn poll(&self) -> Result<Samples> {
        let ref mut m = self.metrics.lock()?;

        let results = vec![Sample::new(m.free.0.clone(), m.free.1.snapshot()),
                           Sample::new(m.used.0.clone(), m.used.1.snapshot())];

        Ok(results)
    }

    fn update(&self) -> BoxFuture<(), Error> {
        let m = self.metrics.clone();

        Box::new(future::lazy(move || {
            let result: Result<()> = m.lock()
                .map_err(Into::into)
                .and_then(|mut locked| locked.update());

            future::result(result)
        }))
    }

    fn next_update(&self) -> Duration {
        self.next_update
    }
}

pub fn input(_id: &str, _config: &toml::Table) -> Result<Box<Input>> {
    Ok(Box::new(Cpu::new()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snapshot() {}
}
