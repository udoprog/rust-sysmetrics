use serde;
use ::plugin::*;
use ::errors::*;
use std::sync::Arc;
use std::fs;
use std::io::Read;
use toml;
use num_cpus;

pub struct Config {
    /// Number of threads to configure in thread pool.
    threads: usize,
    /// If the threads option is per cpu or not.
    threads_per_cpu: bool,
}

/// Model used to parse configuration file.
#[derive(Deserialize, Debug)]
pub struct ConfigIn {
    threads: Option<usize>,
    threads_per_cpu: Option<bool>,
}

pub type PluginSetup = Fn(&Config, &PluginRegistry, &PluginFramework)
                          -> Result<(Vec<Arc<Box<InputInstance>>>, Vec<Box<OutputInstance>>)>;

impl Config {
    pub fn new() -> Config {
        Config {
            threads: 4,
            threads_per_cpu: false,
        }
    }

    pub fn threads(&self) -> usize {
        if self.threads_per_cpu {
            return num_cpus::get() * self.threads;
        }

        self.threads
    }
}

fn load_instance<Entry, Instance, Load, Plugin, Setup>(plugin_section: toml::Value,
                                                       load: Load,
                                                       setup: Setup)
                                                       -> Result<Instance>
    where Entry: Fn(&toml::Table) -> Result<Plugin>,
          Load: Fn(&String) -> Option<Entry>,
          Setup: Fn(Plugin) -> Result<Instance>
{
    let plugin_table: toml::Table = toml::decode(plugin_section).ok_or(ErrorKind::TomlDecode)?;

    let plugin_type: String = plugin_table.get("type")
        .map(Clone::clone)
        .and_then(toml::decode)
        .ok_or(ErrorKind::MissingField("type".to_owned()))?;

    let entry = load(&plugin_type).ok_or(ErrorKind::MissingPlugin(plugin_type))?;

    let plugin = entry(&plugin_table)?;

    setup(plugin)
}

fn load_section<Entry, Instance, Load, Plugin, Setup>(section: &toml::Value,
                                                      load: Load,
                                                      setup: Setup)
                                                      -> Result<Vec<Instance>>
    where Entry: Fn(&toml::Table) -> Result<Plugin>,
          Load: Fn(&String) -> Option<Entry>,
          Setup: Fn(Plugin) -> Result<Instance>
{
    let mut values: Vec<Instance> = Vec::new();

    let table: toml::Table = toml::decode(section.clone()).ok_or(ErrorKind::TomlDecode)?;

    for (_id, plugin_section) in table {
        values.push(load_instance(plugin_section.clone(), &load, &setup).chain_err(|| {
            ErrorKind::ConfigSection(_id)
        })?);
    }

    Ok(values)
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

    if let Some(threads_per_cpu) = config_in.threads_per_cpu {
        config.threads_per_cpu = threads_per_cpu
    }

    let mut input_sections = Vec::new();
    let mut output_sections = Vec::new();

    if let Some(i) = root.get("in") {
        input_sections.push(i.clone());
    }

    if let Some(o) = root.get("out") {
        output_sections.push(o.clone());
    }

    Ok(Box::new(move |_config, plugins, framework| {
        let mut inputs: Vec<Arc<Box<InputInstance>>> = Vec::new();
        let mut outputs: Vec<Box<OutputInstance>> = Vec::new();

        for i in input_sections.iter() {
            let loaded = load_section(
                &i,
                |plugin_type| plugins.get_input(plugin_type),
                |plugin| plugin.setup(framework).map(Arc::new)).chain_err(|| {
                ErrorKind::ConfigSection("in".to_owned())
            })?;

            inputs.extend(loaded);
        }

        for o in output_sections.iter() {
            let loaded = load_section(
                &o,
                |plugin_type| plugins.get_output(plugin_type),
                |plugin| plugin.setup(framework)).chain_err(|| {
                ErrorKind::ConfigSection("out".to_owned())
            })?;

            outputs.extend(loaded);
        }

        Ok((inputs, outputs))
    }))
}
