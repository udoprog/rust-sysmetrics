use ::errors::*;
use futures::*;
use futures_cpupool::CpuPool;
use metric::MetricId;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use toml;

pub type InputEntry = fn(&toml::Table) -> Result<Box<Input>>;
pub type OutputEntry = fn(&toml::Table) -> Result<Box<Output>>;

pub struct PluginRegistry {
    input: HashMap<String, InputEntry>,
    output: HashMap<String, OutputEntry>,
}

impl PluginRegistry {
    pub fn new(input: HashMap<String, InputEntry>,
               output: HashMap<String, OutputEntry>)
               -> PluginRegistry {
        PluginRegistry {
            input: input,
            output: output,
        }
    }

    pub fn get_input(&self, plugin_type: &String) -> Option<&InputEntry> {
        self.input.get(plugin_type)
    }

    pub fn get_output(&self, plugin_type: &String) -> Option<&OutputEntry> {
        self.output.get(plugin_type)
    }

    pub fn input_types<'a>(&'a self) -> impl Iterator<Item = &'a String> + 'a {
        self.input.keys()
    }

    pub fn output_types<'a>(&'a self) -> impl Iterator<Item = &'a String> + 'a {
        self.output.keys()
    }
}

/// A single data sample.
#[derive(Debug)]
pub struct Sample {
    metric_id: Arc<MetricId>,
    value: f64,
}

impl Sample {
    pub fn new(metric_id: Arc<MetricId>, value: f64) -> Sample {
        Sample {
            metric_id: metric_id,
            value: value,
        }
    }
}

pub type Samples = Vec<Sample>;

pub struct PluginFramework {
    pub cpupool: Arc<CpuPool>,
}

pub trait InputInstance: fmt::Debug + Send + Sync {
    /// Poll the state of the plugin instance.
    ///
    /// This is completely independent of the update cycle.
    fn poll(&self) -> Result<Samples> {
        Ok(Vec::new())
    }

    /// Update the state of the plugin instance.
    ///
    /// Returns a future since the operation could be potentially long-running.
    ///
    /// Blocked futures will prevent additional updates from being scheduled until the previous one
    /// has been resolved.
    fn update(&self) -> BoxFuture<(), Error> {
        future::ok(()).boxed()
    }

    /// Get the duration until the next update should be called.
    fn next_update(&self) -> Duration {
        Duration::from_millis(0)
    }
}

pub trait OutputInstance: fmt::Debug + Send + Sync {
    fn feed(&self, sample: &Sample);
}

pub trait Input: fmt::Debug {
    fn setup(&self, framework: &PluginFramework) -> Result<Box<InputInstance>>;
}

pub trait Output: fmt::Debug {
    fn setup(&self, framework: &PluginFramework) -> Result<Box<OutputInstance>>;
}
