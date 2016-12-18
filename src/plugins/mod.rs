mod cpu;
mod disk;
mod load;

#[cfg(feature = "http")]
mod http_poller;

use std::collections::HashMap;
use ::plugin;

pub fn load_plugins() -> HashMap<String, plugin::Entry> {
    let mut m: HashMap<String, plugin::Entry> = HashMap::new();

    m.insert("disk".to_owned(), disk::entry);
    m.insert("cpu".to_owned(), cpu::entry);
    m.insert("load".to_owned(), load::entry);

    #[cfg(feature = "http")]
    m.insert("http_poller".to_owned(), http_poller::entry);

    m
}
