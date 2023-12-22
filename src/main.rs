#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(naked_functions)]
#![feature(stdsimd)]
#![allow(unreachable_code)]
#![feature(riscv_ext_intrinsics)]
#![feature(strict_provenance)]
#![feature(slice_first_last_chunk)]
#![feature(asm_const)]
#![feature(ptr_metadata)]
#![feature(const_mut_refs)]
#![feature(new_uninit)]

extern crate alloc;

mod arch;
pub mod common;
mod config;
mod device;
mod mm;
mod task;

use crate::arch::trap::plic::plic_init;
use crate::arch::trap::{disable_irq_s, enable_irq_s};
use crate::device::console::uart_init;
use arch::cpu::dump_features;
use config::LOGO_STR;
use core::panic::PanicInfo;

pub fn kernel_main() -> ! {
    disable_irq_s();
    plic_init();
    uart_init();
    pr_notice!("{}\n\r", LOGO_STR);
    dump_features();
    mm::init_heap();
    mm::setup_mmu();
    device::pci::init_pci();
    device::pci::find_virt();
    task::init_task();
    enable_irq_s();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pr_err!("panic: {:?}", info);
    loop {
        unsafe { core::arch::riscv64::wfi() };
    }
}
