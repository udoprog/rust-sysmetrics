use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct MetricId {
    key: Option<String>,
    tags: Arc<HashMap<String, String>>
}

impl MetricId {
    pub fn new() -> MetricId {
        MetricId { key: None, tags: Arc::new(HashMap::new()) }
    }

    pub fn new_with_key(key: &str) -> MetricId {
        MetricId { key: Some(key.to_owned()), tags: Arc::new(HashMap::new()) }
    }

    pub fn key(&self, key: &str) -> MetricId {
        MetricId { key: Some(key.to_owned()), tags: self.tags.clone() }
    }

    pub fn tag(&self, key: &str, value: &str) -> MetricId {
        let mut new_tags = (*self.tags).clone();
        new_tags.insert(key.to_owned(), value.to_owned());
        MetricId { key: self.key.clone(), tags: Arc::new(new_tags) }
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

        for (k, v) in &*self.tags {
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
        let m2 = m.tag("host", "foobar");
        let m3 = m.tag("host", "foobar");

        assert!(m != m2, "m = {}, m2 = {}", m, m2);
        assert!(m3 == m2, "m3 = {}, m2 = {}", m3, m2);

        let mut d: HashMap<MetricId, String> = HashMap::new();
        d.insert(m2, "lol".to_owned());
    }
}
