use serde;
use ::plugin::*;
use ::errors::*;
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::fs;
use std::io::Read;
use toml;
use num_cpus;
use futures_cpupool::*;
use tokio_core;
use std::time::Duration;

pub struct Config {
    /// Number of threads to configure in thread pool.
    threads: usize,
    /// If the threads option is per cpu or not.
    threads_per_cpu: bool,
    /// Interval at which to perform updates.
    pub update_interval: Duration,
    /// Interval at which to perform polling.
    pub poll_interval: Duration,
}

/// Model used to parse configuration file.
/// Same as Config, but with optional fields to allow leaving them unspecified.
#[derive(Deserialize, Debug)]
pub struct ConfigIn {
    threads: Option<usize>,
    threads_per_cpu: Option<bool>,
    update_interval: Option<Duration>,
    poll_interval: Option<Duration>,
}

pub struct PartialPluginContext {
    cpupool: Arc<CpuPool>,
    core: Rc<RefCell<tokio_core::reactor::Core>>,
}

impl PartialPluginContext {
    pub fn new(cpupool: Arc<CpuPool>,
               core: Rc<RefCell<tokio_core::reactor::Core>>)
               -> PartialPluginContext {
        PartialPluginContext {
            cpupool: cpupool,
            core: core,
        }
    }

    fn build<'a>(&self, id: &'a String, config: &'a toml::Value) -> PluginContext<'a> {
        PluginContext {
            id: id,
            config: config,
            cpupool: self.cpupool.clone(),
            core: self.core.clone(),
        }
    }
}

pub type PluginSetup = Fn(&Config, &PluginRegistry, &PartialPluginContext)
                          -> Result<(Vec<Arc<Box<InputInstance>>>, Vec<Box<OutputInstance>>)>;

impl Config {
    pub fn new() -> Config {
        // defaults
        Config {
            threads: 4,
            threads_per_cpu: false,
            update_interval: Duration::new(1, 0),
            poll_interval: Duration::new(10, 0),
        }
    }

    pub fn threads(&self) -> usize {
        if self.threads_per_cpu {
            return num_cpus::get() * self.threads;
        }

        self.threads
    }
}

fn load_instance<Entry, Instance, Load, Plugin, Setup>(id: &String,
                                                       plugin_section: toml::Value,
                                                       load: Load,
                                                       setup: Setup)
                                                       -> Result<Instance>
    where Entry: Fn() -> Result<Plugin>,
          Load: Fn(&String) -> Option<Entry>,
          Setup: Fn(Plugin, &String) -> Result<Instance>
{
    let plugin_table: toml::Table = toml::decode(plugin_section).ok_or(ErrorKind::TomlDecode)?;

    let plugin_type: String = plugin_table.get("type")
        .map(Clone::clone)
        .and_then(toml::decode)
        .ok_or(ErrorKind::MissingField("type".to_owned()))?;

    let entry = load(&plugin_type).ok_or(ErrorKind::MissingPlugin(plugin_type))?;

    let plugin = entry()?;

    setup(plugin, id)
}

fn load_section<Entry, Instance, Load, Plugin, Setup>(section: &toml::Value,
                                                      load: Load,
                                                      setup: Setup)
                                                      -> Result<Vec<Instance>>
    where Entry: Fn() -> Result<Plugin>,
          Load: Fn(&String) -> Option<Entry>,
          Setup: Fn(Plugin, &String) -> Result<Instance>
{
    let mut values: Vec<Instance> = Vec::new();

    let table: toml::Table = toml::decode(section.clone()).ok_or(ErrorKind::TomlDecode)?;

    for (id, plugin_section) in table {
        values.push(load_instance(&id, plugin_section.clone(), &load, &setup).chain_err(|| {
            ErrorKind::ConfigSection(id)
        })?);
    }

    Ok(values)
}

/// Read optional fields from input configuration.
macro_rules! read_config {
    ( $config:ident, $config_in:ident, [$($field:ident),*] ) => {
    $(
        if let Some($field) = $config_in.$field {
            $config.$field = $field;
        }
    )*
    };
}

pub fn load_config(config: &mut Config, path: &String) -> Result<Box<PluginSetup>> {
    let mut file = fs::File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut parser = toml::Parser::new(&mut content);

    let root = match parser.parse() {
        Some(value) => value,
        None => return Err(ErrorKind::TomlParse(parser.errors).into()),
    };

    let config_in: ConfigIn = {
        let mut decoder = toml::Decoder::new(toml::Value::Table(root.clone()));
        serde::Deserialize::deserialize(&mut decoder)
    }?;

    if let Some(threads) = config_in.threads {
        if threads <= 0 {
            return Err(ErrorKind::ConfigField("threads".to_owned(),
                                              "must be a positive number".to_owned())
                .into());
        }

        config.threads = threads;
    }

    read_config!(config,
                 config_in,
                 [threads_per_cpu, update_interval, poll_interval]);

    let mut input_configs = Vec::new();
    let mut output_configs = Vec::new();

    if let Some(i) = root.get("in") {
        input_configs.push(i.clone());
    }

    if let Some(o) = root.get("out") {
        output_configs.push(o.clone());
    }

    Ok(Box::new(move |_config, plugins, partial_context| {
        let mut inputs: Vec<Arc<Box<InputInstance>>> = Vec::new();
        let mut outputs: Vec<Box<OutputInstance>> = Vec::new();

        for i in input_configs.iter() {
            let loaded = load_section(&i,
                                      |plugin_type| plugins.get_input(plugin_type),
                                      |plugin, id| {
                                          plugin.setup(partial_context.build(id, &i)).map(Arc::new)
                                      }).chain_err(|| ErrorKind::ConfigSection("in".to_owned()))?;

            inputs.extend(loaded);
        }

        for o in output_configs.iter() {
            let loaded = load_section(&o,
                                      |plugin_type| plugins.get_output(plugin_type),
                                      |plugin, id| {
                                          plugin.setup(partial_context.build(id, &o))
                                      }).chain_err(|| ErrorKind::ConfigSection("out".to_owned()))?;

            outputs.extend(loaded);
        }

        Ok((inputs, outputs))
    }))
}
