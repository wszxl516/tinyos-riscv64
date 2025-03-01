#![allow(dead_code)]

use super::super::config::{
    CLINT_BASE, CLOCK_HZ, GOLDFISH_RTC_TIME, MTIME_CMP_OFFSET, MTIME_OFFSET, NANO_SECOND,
    RTC_BASE_ADDR,
};
use crate::{reg_read_a, reg_write_a};
use core::fmt::{Display, Formatter};
const MS_PEER_HZ: u64 = CLOCK_HZ / 1000;
static mut TICKS: u64 = 0;

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
        let nsec = reg_read_a!((RTC_BASE_ADDR + GOLDFISH_RTC_TIME), u64);
        Self {
            sec: nsec / NANO_SECOND,
            nsec: nsec % NANO_SECOND,
        }
    }

    pub fn set_time(sec: u64, nsec: u64) {
        let nsec = sec * NANO_SECOND + nsec;
        reg_write_a!((RTC_BASE_ADDR + GOLDFISH_RTC_TIME), nsec >> 32, u64);
    }
}

#[no_mangle]
pub fn setup_timer() {
    let current = reg_read_a!(CLINT_BASE + MTIME_OFFSET, u64);
    reg_write_a!(
        CLINT_BASE + MTIME_CMP_OFFSET,
        current + MS_PEER_HZ * 10,
        u64
    );
    unsafe {
        TICKS += 10;
    }
}

pub fn get_ticks() -> u64 {
    unsafe { TICKS }
}

pub fn disable_timer() {
    let current = reg_read_a!(CLINT_BASE + MTIME_OFFSET, u64);
    reg_write_a!(CLINT_BASE + MTIME_CMP_OFFSET, current + u64::MAX, u64);
}

pub fn enable_timer() {
    let current = reg_read_a!(CLINT_BASE + MTIME_OFFSET, u64);
    reg_write_a!(
        CLINT_BASE + MTIME_CMP_OFFSET,
        current + MS_PEER_HZ * 10,
        u64
    );
}
