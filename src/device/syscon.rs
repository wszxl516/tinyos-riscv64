use crate::{config::SYSCON_BASE, reg_write_a};
const POWEROFF_CODE: u32 = 0x5555;
const REBOOT_CODE: u32 = 0x7777;
pub fn poweroff() {
    reg_write_a!(SYSCON_BASE, POWEROFF_CODE, u32);
}
pub fn reboot() {
    reg_write_a!(SYSCON_BASE, REBOOT_CODE, u32);
}
