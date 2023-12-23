#![allow(dead_code)]

mod exception;
mod interrupt;
pub(crate) mod plic;
pub mod trap;

use crate::arch::timer::enable_timer;
use crate::{reg_clear_bit_p, reg_read_p, reg_update_p, reg_write_g, reg_write_p};
use core::arch::asm;
pub use trap::{disable_irq_m, disable_irq_s, enable_irq_m, enable_irq_s};

pub fn setup_trap() {
    disable_irq_m();
    //set MPP to 1 (supervisor mode)
    //mstatus.SIE = 1
    //enable fpu
    reg_clear_bit_p!(mstatus, 0b11 << 11);
    reg_update_p!(mstatus, 1 << 13 | 1 << 11 | 1 << 1);
    // Reset satp  disable mmu
    reg_write_p!(satp, 0);

    // delegate all interrupts and exceptions to supervisor mode.
    reg_write_p!(medeleg, 0xffff);
    reg_write_p!(mideleg, 0xffff);
    //external interrupt-enable | timer interrupt-enable | software interrupt-enable
    reg_update_p!(sie, 1 << 9 | 1 << 5 | 1 << 1);
    reg_update_p!(mie, 1 << 9 | 1 << 5 | 1 << 1);

    // keep each CPU's hartid in its tp register, for cpuid().
    let value = reg_read_p!(mhartid);
    reg_write_g!(tp, value);

    // set trap handler
    reg_write_p!(mtvec, trap::m_trap_handler as usize);
    reg_write_p!(stvec, trap::s_trap_handler as usize);

    /*machine mode exception return pointer*/
    reg_write_p!(mepc, crate::kernel_main as usize);
    /*all physical memory */
    // 7	6-5	 4-3	2	1	0
    // L	0	 A	    X	W	R
    reg_write_p!(pmpcfg0, 1 << 0 | 1 << 1 | 1 << 2 | 0b11 << 3);
    reg_write_p!(pmpaddr0, usize::MAX >> 10);
    //supervisor mode trap stack
    reg_write_p!(sscratch, trap::S_TRAP_FRAMES.addr() as usize);
    reg_write_p!(mscratch, trap::M_TRAP_FRAMES.addr() as usize);

    //enable machine mode timer
    enable_timer();
    // enable all interrupt and exception
    enable_irq_m();
    unsafe { asm!("mret") }
}
