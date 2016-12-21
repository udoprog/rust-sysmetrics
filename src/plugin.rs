use ::errors::*;
use futures::*;
use futures_cpupool::CpuPool;
use metric::MetricId;
use std::fmt::Debug;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use tokio_timer::Timer;
use toml;

#[derive(Debug)]
pub struct Sample {
    metric_id: Arc<MetricId>,
    value: f64
}

impl Sample {
    pub fn new(metric_id: Arc<MetricId>, value: f64) -> Sample {
        Sample { metric_id: metric_id, value: value }
    }
}

pub type Samples = Vec<Sample>;

pub struct PluginFramework {
    pub cpupool: Rc<CpuPool>
}

pub trait PluginInstance: Debug {
    /// Poll the state of the plugin instance.
    fn poll(&self) -> Result<Samples> {
        Ok(Vec::new())
    }

    /// Update the state of the plugin instance.
    fn update(&self) -> BoxFuture<(), Error> {
        future::ok(()).boxed()
    }

    /// Get the duration until the next update should be called.
    fn next_update(&self) -> Duration {
        Duration::from_millis(0)
    }
}

pub trait Plugin: Debug  {
    fn key(&self) -> &str;

    fn setup(&self, framework: &PluginFramework) -> Result<Box<PluginInstance>>;
}

#[derive(Debug)]
pub enum Control {
    Exit
}

pub type Entry = fn(String, toml::Value) -> Result<Box<Plugin>>;
