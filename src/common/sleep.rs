use super::super::config::{CLINT_BASE, CLOCK_HZ, MTIME_OFFSET};
use crate::reg_read_a;
#[inline(always)]
pub fn get_sys_time() -> u64 {
    reg_read_a!(CLINT_BASE + MTIME_OFFSET, u64)
}

#[inline(always)]
pub fn arch_usleep(us: u64) -> u64 {
    let start_time = get_sys_time();
    let end_time = start_time + us * (CLOCK_HZ / 1_000_000);
    while get_sys_time() < end_time {}
    us
}
pub fn sleep_ms(seconds: u64) {
    arch_usleep(seconds * 1000);
}
