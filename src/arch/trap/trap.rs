#![allow(dead_code)]

use super::exception::{exception_handler, Exception};
use super::interrupt::{interrupt_handler, Interrupt};
use crate::{impl_field_name_and_value, get_bits, impl_numeric_enum, reg_read_p, reg_write_p};
use core::arch::global_asm;

global_asm!(include_str!("macros.S"), include_str!("trap.S"));
extern "C" {
    pub fn m_trap_handler();
    pub fn s_trap_handler();
}

#[inline(always)]
pub fn enable_irq_m() {
    reg_write_p!(mie, 0xfff);
}

#[inline(always)]
pub fn disable_irq_m() {
    reg_write_p!(mie, 0);
}

#[inline(always)]
pub fn enable_irq_s() {
    reg_write_p!(sie, 0xfff);
}

#[inline(always)]
pub fn disable_irq_s() {
    reg_write_p!(sie, 0);
}

// Register 	ABI Name 	Description 	                Saver
// x0 	        zero 	    Hard-wired zero 	            -
// x1 	        ra 	        Return address 	                Caller
// x2 	        sp 	        Stack pointer 	                Callee
// x3 	        gp 	        Global pointer 	                -
// x4 	        tp 	        Thread pointer 	                -
// x5 	        t0 	        Temporary/   	                Caller
// x6-7 	    t1-2 	    Temporaries 	                Caller
// x8 	        s0/fp 	    Saved regsiter/frame         	Callee
// x9 	        s1 	        Saved register 	                Callee
// x10-11 	    a0-1 	    arguments/return                Caller
// x12-17 	    a2-7 	    arguments 	                    Caller
// x18-27 	    s2-11 	    Saved registers 	            Callee
// x28-31 	    t3-6 	    Temporaries 	                Caller

impl_field_name_and_value! {
    usize,
    #[repr(C, align(8))]
    #[derive(Debug, Clone, Default)]
    pub struct Context {
        pub ra,
        pub sp,
        pub gp,
        pub tp,
        pub t0,
        pub t1,
        pub t2,
        pub s0,
        pub s1,
        pub a0,
        pub a1,
        pub a2,
        pub a3,
        pub a4,
        pub a5,
        pub a6,
        pub a7,
        pub s2,
        pub s3,
        pub s4,
        pub s5,
        pub s6,
        pub s7,
        pub s8,
        pub s9,
        pub s10,
        pub s11,
        pub t3,
        pub t4,
        pub t5,
        pub t6,
        pub pc,
    }
}

impl Context {
    pub const fn empty() -> Self {
        Self {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            pc: 0,
        }
    }
    pub fn addr(&self) -> usize {
        self as *const Self as usize
    }
    #[inline]
    pub fn replace(&mut self, data: &Self) {
        self.mut_array().copy_from_slice(data.array());
    }
    #[inline]
    pub fn mut_array(&mut self) -> &mut [usize; 32] {
        unsafe { core::mem::transmute(self) }
    }
    #[inline]
    pub fn array(&self) -> &[usize; 32] {
        unsafe { core::mem::transmute(self) }
    }
}

#[derive(Debug)]
pub struct Regs {
    pub context: &'static mut Context,
    pub ra: usize,
    pub epc: usize,
    pub tval: usize,
}

#[derive(Debug)]
pub struct Trap {
    regs: Regs,
    cause: usize,
}

impl_numeric_enum! {
    u32,
    #[derive(Debug, Clone, Copy)]
    pub ExceptionType [
        Exception = 0,
        Interrupt = 1,
    ]
}

impl Trap {
    pub fn new(sp: &'static mut Context, ra: usize) -> Self {
        Self {
            regs: Regs {
                context: sp,
                ra,
                epc: reg_read_p!(sepc),
                tval: reg_read_p!(stval),
            },
            cause: reg_read_p!(scause),
        }
    }
    #[inline(always)]
    pub fn exception_type(&self) -> ExceptionType {
        ExceptionType::from_value(get_bits!(self.cause, 63, 1) as u32)
    }
    #[inline(always)]
    pub fn exception_code(&self) -> u32 {
        get_bits!(self.cause, 0, 63) as u32
    }
}

#[no_mangle]
pub fn handle_trap(stack: &'static mut Context, ra: usize) {
    let mut trap = Trap::new(stack, ra);
    match trap.exception_type() {
        ExceptionType::Interrupt => interrupt_handler(
            Interrupt::from_value(trap.exception_code()),
            trap.regs.context,
        ),

        ExceptionType::Exception => {
            exception_handler(Exception::from_value(trap.exception_code()), &mut trap.regs)
        }

        ExceptionType::Unknown(v) => {
            panic!("Unknown ExceptionType: {:#x}\n", v)
        }
    }
}
