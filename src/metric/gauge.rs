use std::f64;
use std::mem;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct Gauge {
    value: AtomicU64,
}

impl Gauge {
    pub fn new() -> Gauge {
        let bits = unsafe { mem::transmute(f64::NAN) };
        Gauge { value: AtomicU64::new(bits) }
    }

    pub fn clear(&mut self) {
        let bits = unsafe { mem::transmute(f64::NAN) };
        self.value.store(bits, Ordering::Relaxed);
    }

    pub fn set(&mut self, value: f64) {
        let bits = unsafe { mem::transmute(value) };
        self.value.store(bits, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> f64 {
        let bits = self.value.load(Ordering::Relaxed);
        unsafe { mem::transmute(bits) }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    use std::f64;

    #[test]
    fn snapshot() {
        let mut c: Gauge = Gauge::new();

        let s1 = c.snapshot();
        c.set(1f64);
        let s2 = c.snapshot();

        assert!(f64::is_nan(s1));
        assert!(s2 == 1f64);
    }
}
