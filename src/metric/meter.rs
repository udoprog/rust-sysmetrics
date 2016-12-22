#![allow(dead_code)]

// Forked from https://github.com/ekarlso/rust-metrics/blob/4bf5446/src/meter.rs

use super::ewma;
use time::{Timespec, get_time};

const WINDOW: [f64; 3] = [1f64, 5f64, 15f64];

#[derive(Debug)]
pub struct MeterSnapshot {
    count: i64,
    rates: [f64; 3],
    mean: f64,
}

#[derive(Debug)]
pub struct Meter {
    count: i64,
    rates: [f64; 3],
    mean: f64,
    ewma: Vec<ewma::EWMA>,
    start: Timespec,
}

impl Meter {
    pub fn new() -> Meter {
        let ewma = vec![ewma::EWMA::new(1f64), ewma::EWMA::new(5f64), ewma::EWMA::new(15f64)];
        Meter {
            count: 0i64,
            rates: [0f64, 0f64, 0f64],
            mean: 0f64,
            ewma: ewma,
            start: get_time(),
        }
    }

    fn snapshot(&self) -> MeterSnapshot {
        MeterSnapshot {
            count: self.count,
            rates: self.rates,
            mean: self.mean,
        }
    }

    fn mark(&mut self, n: i64) {
        self.count += n;

        for i in 0..self.ewma.len() {
            self.ewma[i].update(n as usize);
        }

        self.update_snapshot();
    }

    fn tick(&mut self) {
        for i in 0..self.ewma.len() {
            self.ewma[i].tick();
        }

        self.update_snapshot();
    }

    /// Return the given EWMA for a rate like 1, 5, 15 minutes
    fn rate(&self, rate: f64) -> f64 {
        if let Some(pos) = WINDOW.iter().position(|w| *w == rate) {
            return self.rates[pos];
        }

        0f64
    }

    /// Return the mean rate
    fn mean(&self) -> f64 {
        self.mean
    }

    fn count(&self) -> i64 {
        self.count
    }

    fn update_snapshot(&mut self) {
        for i in 0..WINDOW.len() {
            self.rates[i] = self.ewma[i].rate();
        }

        let diff = get_time() - self.start;
        self.mean = self.count as f64 / diff.num_seconds() as f64;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero() {
        let mut m: Meter = Meter::new();
        let s: MeterSnapshot = m.snapshot();

        assert_eq!(s.count, 0);
    }

    #[test]
    fn non_zero() {
        let mut m: Meter = Meter::new();
        m.mark(3);

        let s: MeterSnapshot = m.snapshot();

        assert_eq!(s.count, 3);
    }

    #[test]
    fn snapshot() {
        let mut m: Meter = Meter::new();
        m.mark(1);
        m.mark(1);

        let s = m.snapshot();
        m.mark(1);

        assert_eq!(s.count, 2);
        assert_eq!(m.snapshot().count, 3);
    }

    // Test that decay works correctly
    #[test]
    fn decay() {
        let mut m: Meter = Meter::new();

        m.tick();
    }
}
