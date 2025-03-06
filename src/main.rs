#![no_std]
#![no_main]
#![feature(naked_functions)]
#![allow(unreachable_code)]
#![feature(riscv_ext_intrinsics)]
#![feature(ptr_metadata)]
#![allow(static_mut_refs)]
#![feature(optimize_attribute)]
extern crate alloc;

mod arch;
pub mod common;
mod config;
mod device;
mod mm;
mod task;
mod tasks;
use crate::arch::trap::{disable_irq_s, enable_irq_s};
use arch::cpu::dump_features;
use config::LOGO_STR;
use core::panic::PanicInfo;
use device::console::uart_irq_init;
pub fn kernel_main() -> ! {
    disable_irq_s();
    uart_irq_init();
    pr_notice!("{}\n\r", LOGO_STR);
    dump_features();
    mm::init_heap();
    mm::setup_mmu();
    device::pci::init_pci();
    device::pci::find_virt();
    task::task_add(1, "demo0", tasks::demo0, 5);
    task::task_add(2, "demo1", tasks::demo1, 10);
    task::task_add(3, "shell", tasks::shell, 1);
    enable_irq_s();
    loop {
        unsafe { core::arch::riscv64::wfi() };
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pr_err!("panic: {:?}", info);
    loop {
        unsafe { core::arch::riscv64::wfi() };
    }
}
