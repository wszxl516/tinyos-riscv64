#![allow(dead_code)]
use super::{syscall::syscall, trap::Regs};
use crate::impl_numeric_enum;
use crate::{common::symbol::find_symbol, pr_err, reg_read_a};

impl_numeric_enum! {
    u32,
    pub Exception [
        InstructionAddressMisaligned = 0,
        InstructionAccessFault = 1,
        IllegalInstruction = 2,
        BREAKPOINT = 3,
        LoadAddressMisaligned = 4,
        LoadAccessFault = 5,
        StoreAmoAddressMisaligned = 6,
        StoreAmoAccessFault = 7,
        EnvironmentCallFromUMode = 8,
        EnvironmentCallFromSMode = 9,
        //10 RESERVED
        EnvironmentCallFromMMode = 11,
        InstructionPageFault = 12,
        LoadPageFault = 13,
        //14 RESERVED
        StoreAmoPageFault = 15,
        //16–23 RESERVED
        //24–31 DESIGNATED FOR CUSTOM USE
        //32–47 RESERVED
        //48–63 DESIGNATED FOR CUSTOM USE
        //≥64 RESERVED
    ]
}

fn dump_stack(regs: &Regs) {
    pr_err!("\n");
    pr_err!("call stack: \n\t#1: {:#x} ", regs.epc);
    match find_symbol(regs.epc) {
        Some(sym) => pr_err!("({}+{})\n", sym.name, regs.epc - sym.addr),
        None => pr_err!("\n"),
    }
    pr_err!("\t#0: {:#x} ", regs.ra);
    match find_symbol(regs.ra) {
        Some(sym) => pr_err!("({}+{})\n", sym.name, regs.ra - sym.addr),
        None => pr_err!("\n"),
    }
    pr_err!("tval: {:#x}\n", regs.tval);
    if regs.epc != 0 {
        pr_err!("code: ");
        let current_code = reg_read_a!(regs.epc, u16);
        let code = if current_code & 0b11 == 0b11 {
            (current_code as u32) | ((reg_read_a!(regs.epc + 2, u16) as u32) << 16)
        }
        // compressed instruction
        else {
            current_code as u32
        };
        pr_err!("{:#04x} ", code);
    }
    pr_err!("\n");
    pr_err!("{}", regs.context);
    pr_err!("\n");
    pr_err!("\n");
}

pub fn exception_handler(exception: Exception, regs: &mut Regs) {
    match exception {
        Exception::InstructionAddressMisaligned => pr_err!("Instruction address misaligned."),
        Exception::InstructionAccessFault => pr_err!("Instruction access fault."),
        Exception::IllegalInstruction => pr_err!("Illegal instruction."),
        Exception::BREAKPOINT => pr_err!("Breakpoint."),
        Exception::LoadAddressMisaligned => pr_err!("Load address misaligned."),
        Exception::LoadAccessFault => pr_err!("Load access fault."),
        Exception::StoreAmoAddressMisaligned => pr_err!(" Store/AMO address misaligned."),
        Exception::StoreAmoAccessFault => pr_err!("Store/AMO access fault."),
        Exception::EnvironmentCallFromUMode => pr_err!("Environment call from U-mode."),
        Exception::EnvironmentCallFromSMode => {
            // pr_err!("Environment call from S-mode.");
            // pc should add 4
            regs.context.pc += 4;
            syscall(regs);
            return;
        }
        Exception::EnvironmentCallFromMMode => pr_err!("Environment call from M-mode."),
        Exception::InstructionPageFault => pr_err!("Instruction page fault."),
        Exception::LoadPageFault => pr_err!("Load page fault."),
        Exception::StoreAmoPageFault => pr_err!("Store/AMO page fault."),
        Exception::Unknown(v) => {
            panic!("Unknown Exception: {:#x}\n", v)
        }
    }
    dump_stack(regs);
    loop {}
}
