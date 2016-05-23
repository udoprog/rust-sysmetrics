use std::f64;

#[derive(Copy, Clone, Debug)]
pub struct Gauge {
    pub value: f64
}

impl Gauge {
    pub fn new() -> Gauge {
        Gauge { value: f64::NAN }
    }

    fn clear(&mut self) {
        self.value = f64::NAN;
    }

    fn set(&mut self, value: f64) {
        self.value = value;
    }

    fn snapshot(self) -> f64 {
        self.value
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
