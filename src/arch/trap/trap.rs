#![allow(dead_code)]

use super::exception::{exception_handler, Exception};
use super::interrupt::{interrupt_handler, Interrupt};
use crate::display_with_field_name;
use crate::{get_bits, reg_read_p, reg_write_p};
use core::arch::global_asm;
pub static mut S_TRAP_FRAMES: Context = Context::empty();
pub static mut M_TRAP_FRAMES: Context = Context::empty();

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

pub type Reg = usize;
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

display_with_field_name! {
    "{: <3} = {:#018x}",
    #[repr(C, align(16))]
    #[derive(Debug, Clone, Default)]
    pub struct Context {
        pub ra: Reg,
        pub sp: Reg,
        gp: Reg,
        tp: Reg,
        t0: Reg,
        t1: Reg,
        t2: Reg,
        s0: Reg,
        s1: Reg,
        a0: Reg,
        a1: Reg,
        a2: Reg,
        a3: Reg,
        a4: Reg,
        a5: Reg,
        a6: Reg,
        a7: Reg,
        s2: Reg,
        s3: Reg,
        s4: Reg,
        s5: Reg,
        s6: Reg,
        s7: Reg,
        s8: Reg,
        s9: Reg,
        s10: Reg,
        s11: Reg,
        t3: Reg,
        t4: Reg,
        t5: Reg,
        t6: Reg,
        pub pc: Reg,
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
    pub fn mut_array(&mut self) -> &mut [Reg; 32] {
        unsafe { core::mem::transmute(self) }
    }
    #[inline]
    pub fn array(&self) -> &[Reg; 32] {
        unsafe { core::mem::transmute(self) }
    }
}

#[derive(Debug)]
pub struct Regs {
    pub context: Context,
    pub ra: usize,
    pub epc: usize,
    pub tval: usize,
}

#[derive(Debug)]
pub struct Trap {
    regs: Regs,
    cause: usize,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ExceptionType {
    Exception = 0,
    Interrupt = 1,
}

impl ExceptionType {
    pub fn from_u32(value: u32) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl Trap {
    pub fn new(sp: &mut Context, ra: usize) -> Self {
        Self {
            regs: Regs {
                context: sp.clone(),
                ra,
                epc: reg_read_p!(sepc),
                tval: reg_read_p!(stval),
            },
            cause: reg_read_p!(scause),
        }
    }
    #[inline(always)]
    pub fn exception_type(&self) -> ExceptionType {
        ExceptionType::from_u32(get_bits!(self.cause, 63, 1) as u32)
    }
    #[inline(always)]
    pub fn exception_code(&self) -> u32 {
        get_bits!(self.cause, 0, 63) as u32
    }
}

#[no_mangle]
pub fn handle_trap(stack: &mut Context, ra: usize) {
    let trap = Trap::new(stack, ra);
    match trap.exception_type() {
        ExceptionType::Interrupt => {
            interrupt_handler(Interrupt::from_u32(trap.exception_code()), stack)
        }

        ExceptionType::Exception => {
            exception_handler(Exception::from_u32(trap.exception_code()), &trap.regs)
        }
    }
}
