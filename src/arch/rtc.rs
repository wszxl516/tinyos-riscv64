#![allow(dead_code)]
use crate::config::RTC_BASE_ADDR;
use core::fmt::{Display, Formatter};
const NSEC_PER_SEC: u64 = 1_000_000_000;
use crate::{reg_read_a, reg_write_a};

pub struct Time {
    sec: u64,
    nsec: u64,
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.sec, self.nsec)
    }
}

impl Time {
    pub fn get_time() -> Self {
        let low = reg_read_a!(RTC_BASE_ADDR, u32);
        let high = reg_read_a!(RTC_BASE_ADDR + 4, u32);
        let nsecs = ((high as u64) << 32) | low as u64;
        Self {
            sec: nsecs / NSEC_PER_SEC,
            nsec: nsecs % NSEC_PER_SEC,
        }
    }

    pub fn set_time(sec: u64, nsec: u64) {
        let nsec = sec * NSEC_PER_SEC + nsec;
        reg_write_a!((RTC_BASE_ADDR), nsec as u32, u32);
        reg_write_a!((RTC_BASE_ADDR + 4), (nsec >> 32) as u32, u32);
    }
}
