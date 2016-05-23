extern crate num;

#[derive(Copy, Clone, Debug)]
pub struct Counter {
    pub value: i64
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
    }

    fn clear(&mut self) {
        self.value = 0;
    }

    fn dec(&mut self, value: i64) {
        self.value = self.value - value;
    }

    fn inc(&mut self, value: i64) {
        self.value = self.value + value;
    }

    fn snapshot(self) -> i64 {
        self.value
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn increment_by_1() {
        let mut c: Counter = Counter::new();
        c.inc(1);
        assert!(c.value == 1);
    }

    #[test]
    fn snapshot() {
        let mut c: Counter = Counter::new();

        let s1 = c.snapshot();
        c.inc(1);
        let s2 = c.snapshot();

        assert!(c.value == 1);
        assert!(s1 == 0);
        assert!(s2 == 1);
    }
}
