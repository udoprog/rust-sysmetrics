mod ewma;
pub mod metric_id;
pub mod counter;
pub mod gauge;
pub mod meter;

pub use self::metric_id::MetricId;
pub use self::metric_id::MetricIdBuilder;
pub use self::gauge::Gauge;

pub fn key(key: &str) -> MetricIdBuilder {
    return MetricId::new_with_key(key);
}
