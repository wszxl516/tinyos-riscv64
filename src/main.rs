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

use crate::arch::trap::{disable_irq_s, enable_irq_s};
use arch::cpu::dump_features;
use common::sleep::sleep_ms;
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
    task::task_add(1, "demo0", demo0, 4);
    task::task_add(2, "demo1", demo1, 4);
    task::task_add(3, "stats", stats, 1);
    pr_notice!("finished task init!\n");
    enable_irq_s();
    loop {}
}

#[optimize(none)]
fn demo1() -> ! {
    loop {
        for x in 0..10 {
            pr_info!("demo1 - {}\n", x);
            sleep_ms(100);
        }
    }
}

#[optimize(none)]
fn demo0() -> ! {
    loop {
        for x in 0..10 {
            pr_notice!("demo0 - {}\n", x);
            sleep_ms(100);
        }
    }
}
#[optimize(none)]
fn stats() -> ! {
    loop {
        pr_warn!(
            "|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}\n",
            "name",
            "pid",
            "priority",
            "total",
            "state"
        );
        disable_irq_s();
        {
            task::each_task(|t| {
                pr_warn!(
                    "|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}\n",
                    t.name,
                    t.id,
                    t.priority,
                    t.total_time,
                    t.state
                )
            });
        }

        enable_irq_s();
        sleep_ms(1000);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pr_err!("panic: {:?}", info);
    loop {
        unsafe { core::arch::riscv64::wfi() };
    }
}
