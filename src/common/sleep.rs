use super::super::config::{
    CLINT_BASE, MTIME_OFFSET
};
use crate::reg_read_a;
pub const CLOCK_FREQ: u64 =  10_000_000;
#[inline(always)]
pub fn get_sys_time() -> u64 {
    reg_read_a!(CLINT_BASE + MTIME_OFFSET, u64)
}
pub fn arch_usleep(us: u64) -> u64 {
    let start_time = get_sys_time();
    let end_time = start_time + us * (CLOCK_FREQ / 1000000);
    while get_sys_time() < end_time {}
    return us;
}
pub fn sleep_ms(seconds: u64) {
    arch_usleep(seconds * 1000);
}