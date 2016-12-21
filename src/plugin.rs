use ::errors::*;
use futures::*;
use futures_cpupool::CpuPool;
use metric::MetricId;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use toml;

/// The kind of the plugin being configured.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
pub enum PluginKind {
    Read, Write
}

impl fmt::Display for PluginKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PluginKind::Read => write!(f, "read"),
            PluginKind::Write => write!(f, "write"),
        }
    }
}

/// The key of the plugin being configured.
#[derive(PartialEq, Debug, Clone)]
pub struct PluginKey {
    pub plugin_kind: PluginKind,
    pub plugin_type: String
}

impl fmt::Display for PluginKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.plugin_kind, self.plugin_type)
    }
}

pub type PluginRegistryKey = (PluginKind, String);
pub type PluginRegistry = HashMap<PluginRegistryKey, Entry>;

/// A single data sample.
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

pub trait PluginInstance: fmt::Debug {
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

pub trait Plugin: fmt::Debug  {
    fn setup(&self, framework: &PluginFramework) -> Result<Box<PluginInstance>>;
}

#[derive(Debug)]
pub enum Control {
    Exit
}

pub type Entry = fn(key: &PluginKey, toml::Value) -> Result<Box<Plugin>>;
