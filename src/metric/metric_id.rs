use std::fmt;

#[derive(Serialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetricIdBuilder {
    key: Option<String>,
    tags: Vec<(String, String)>,
    resource: Vec<(String, String)>,
}

impl MetricIdBuilder {
    pub fn key(mut self, key: &str) -> MetricIdBuilder {
        self.key = Some(key.to_owned());
        self
    }

    pub fn tag(mut self, key: &str, value: &str) -> MetricIdBuilder {
        self.tags.push((key.to_owned(), value.to_owned()));
        self
    }

    pub fn resource(mut self, key: &str, value: &str) -> MetricIdBuilder {
        self.resource.push((key.to_owned(), value.to_owned()));
        self
    }

    pub fn build(&self) -> MetricId {
        MetricId {
            key: self.key.clone(),
            tags: self.tags.clone(),
            resource: self.resource.clone(),
        }
    }
}

#[derive(Serialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetricId {
    key: Option<String>,
    tags: Vec<(String, String)>,
    resource: Vec<(String, String)>,
}

impl MetricId {
    pub fn new() -> MetricIdBuilder {
        MetricIdBuilder {
            key: None,
            tags: Vec::new(),
            resource: Vec::new(),
        }
    }

    pub fn new_with_key(key: &str) -> MetricIdBuilder {
        MetricIdBuilder {
            key: Some(key.to_owned()),
            tags: Vec::new(),
            resource: Vec::new(),
        }
    }
}

impl fmt::Display for MetricId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.key, self.tags, self.resource)
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
