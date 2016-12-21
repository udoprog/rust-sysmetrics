mod cpu;
mod disk;
mod load;

#[cfg(feature = "http")]
mod http_poller;

use ::plugin::*;
use std::collections::HashMap;

pub fn load_plugins() -> PluginRegistry {
    let mut m: PluginRegistry = HashMap::new();

    m.insert((PluginKind::Read, "disk".to_owned()), disk::entry);
    m.insert((PluginKind::Read, "cpu".to_owned()), cpu::entry);
    m.insert((PluginKind::Read, "load".to_owned()), load::entry);

    #[cfg(feature = "http")]
    m.insert((PluginKind::Read, "http_poller".to_owned()), http_poller::entry);

    m
}
