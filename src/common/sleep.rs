#[allow(dead_code)]
use super::syscall::syscall;
use crate::arch::trap::syscall::SyscallNo;
use crate::syscall_args;

fn sys_sleep(us: u64) -> isize {
    syscall(SyscallNo::Sleep.into_value(), syscall_args![us as usize])
}

#[inline(always)]
pub fn sleep_us(us: u64) {
    sys_sleep(us);
}

#[inline(always)]
pub fn sleep_ms(ms: u64) {
    sys_sleep(ms * 1000);
}

#[inline(always)]
pub fn sleep(s: u64) {
    sys_sleep(s * 1000 * 1000);
}
