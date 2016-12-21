use std::str::{self, FromStr};
use nom::{line_ending, space, digit};

named!(pub type_u64<u64>,
       map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

#[derive(Debug, Default, PartialEq)]
pub struct StatCpu {
    /// normal processes executing in user mode
    pub user: u64,
    /// niced processes executing in user mode
    pub nice: u64,
    /// processes executing in kernel space
    pub system: u64,
    /// twiddling thumbs
    pub idle: u64,
    /// waiting for I/O to complete
    pub iowait: u64,
    /// servicing interrupts
    pub irq: u64,
    /// servicing softirqs
    pub softirq: u64,
    /// involuntary wait
    pub steal: u64,
    /// running a normal guest
    pub guest: u64,
    /// running a niced guest
    pub guest_nice: u64,
}

impl StatCpu {
    pub fn total(&self) -> u64 {
        return
            self.user +
            self.nice +
            self.system +
            self.idle +
            self.iowait +
            self.irq +
            self.softirq +
            self.steal +
            self.guest +
            self.guest_nice;
    }

    pub fn used(&self) -> u64 {
        return
            self.user +
            self.nice +
            self.system +
            self.iowait +
            self.irq +
            self.softirq +
            self.steal +
            self.guest +
            self.guest_nice;
    }

    pub fn free(&self) -> u64 {
        return self.idle;
    }
}

named!(pub parse_stat_cpu<StatCpu>,
    chain!(
        cpu: tag!("cpu") ~ space ~
        user: type_u64 ~ space ~
        nice: type_u64 ~ space ~
        system: type_u64 ~ space ~
        idle: type_u64 ~ space ~
        iowait: type_u64 ~ space ~
        irq: type_u64 ~ space ~
        softirq: type_u64 ~ space ~
        steal: type_u64 ~ space ~
        guest: type_u64 ~ space ~
        guest_nice: type_u64 ~ line_ending,
        || {
            StatCpu {
                user: user,
                nice: nice,
                system: system,
                idle: idle,
                iowait: iowait,
                irq: irq,
                softirq: softirq,
                steal: steal,
                guest: guest,
                guest_nice: guest_nice } }));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_loadavg() {
        let cpu_text = b"cpu  347703 107 67084 8538266 10258 0 8753 0 0 0\n";
        let cpu = parse_stat_cpu(cpu_text).to_full_result().unwrap();
        assert_eq!(347703, cpu.user);
    }
}
