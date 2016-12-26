use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct MetricIdBuilder {
    key: Option<String>,
    tags: HashMap<String, String>,
    identity: BTreeSet<String>,
    meaning: BTreeSet<String>,
}

impl MetricIdBuilder {
    pub fn key(mut self, key: &str) -> MetricIdBuilder {
        self.key = Some(key.to_owned());
        self
    }

    pub fn tags(mut self, entries: &[(&str, &str)]) -> MetricIdBuilder {
        for &(key, value) in entries {
            self.tags.insert(key.to_owned(), value.to_owned());
        }

        self
    }

    pub fn identity(mut self, entries: &[&str]) -> MetricIdBuilder {
        for &value in entries {
            self.identity.insert(value.to_owned());
        }

        self
    }

    pub fn meaning(mut self, entries: &[&str]) -> MetricIdBuilder {
        for &value in entries {
            self.meaning.insert(value.to_owned());
        }

        self
    }

    pub fn build(&self) -> MetricId {
        MetricId {
            key: self.key.clone(),
            tags: self.tags.clone(),
            identity: self.identity.clone(),
            meaning: self.meaning.clone(),
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct MetricId {
    key: Option<String>,
    tags: HashMap<String, String>,
    identity: BTreeSet<String>,
    meaning: BTreeSet<String>,
}

impl MetricId {
    pub fn new() -> MetricIdBuilder {
        MetricIdBuilder {
            key: None,
            tags: HashMap::new(),
            identity: BTreeSet::new(),
            meaning: BTreeSet::new(),
        }
    }

    pub fn new_with_key(key: &str) -> MetricIdBuilder {
        MetricIdBuilder {
            key: Some(key.to_owned()),
            tags: HashMap::new(),
            identity: BTreeSet::new(),
            meaning: BTreeSet::new(),
        }
    }
}

impl fmt::Display for MetricId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MetricId({:?})", self.tags)
    }
}

impl Hash for MetricId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);

        for (k, v) in &self.tags {
            k.hash(state);
            v.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_metric_id() {
        let m = MetricId::new();
        let m0 = m.build();
        let m2 = m.clone().tags(&[("host", "foobar")]).build();
        let m3 = m.clone().tags(&[("host", "foobar")]).build();

        assert!(m0 != m2, "m = {}, m2 = {}", m0, m2);
        assert!(m3 == m2, "m3 = {}, m2 = {}", m3, m2);

        let mut d: HashMap<MetricId, String> = HashMap::new();
        d.insert(m2, "lol".to_owned());
    }
}
