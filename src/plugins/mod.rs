mod cpu;
mod disk;
mod load;

#[cfg(feature = "http")]
mod http_poller;

use ::plugin::*;
use std::collections::HashMap;

pub fn load_plugins() -> PluginRegistry {
    let mut input: HashMap<String, InputEntry> = HashMap::new();

    input.insert("disk".to_owned(), disk::input);
    input.insert("cpu".to_owned(), cpu::input);
    input.insert("load".to_owned(), load::input);

    #[cfg(feature = "http")]
    input.insert("http_poller".to_owned(), http_poller::input);

    PluginRegistry::new(input)
}
