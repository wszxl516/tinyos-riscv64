pub mod cpu;
mod entry;
pub mod macros;
pub mod plic;
pub mod rtc;
pub mod timer;
pub mod trap;
pub static mut BOOT_ARGS: [usize; 3] = [0; 3];
