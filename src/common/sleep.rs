use crate::arch::timer::get_ticks;
// #[inline(always)]
#[optimize(none)]
pub fn arch_usleep(us: u64) -> u64 {
    let start_time = get_ticks();
    let end_time = start_time + us;
    loop {
        let current = get_ticks();
        if current >= end_time {
            break;
        }
    }
    us
}
pub fn sleep_ms(ms: u64) {
    arch_usleep(ms * 1000);
}
