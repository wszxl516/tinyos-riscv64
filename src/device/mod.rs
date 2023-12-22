use super::config::BASE_UART;
use core::arch::asm;

pub static mut DTB_ADDR: usize = 0;

pub mod console;
pub mod uart;
pub mod pci;
pub mod virtio;

pub fn get_args() {
    unsafe {
        asm!(
        "mv {addr}, a1",
        addr = out(reg) DTB_ADDR
        );
    }
}
