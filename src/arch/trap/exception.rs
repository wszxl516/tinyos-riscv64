#![allow(dead_code)]

use super::trap::Regs;
use crate::{pr_err, reg_read_a};

#[repr(u32)]
pub enum Exception {
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
}

impl Exception {
    pub fn from_u32(value: u32) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

fn dump_stack(regs: &Regs) {
    pr_err!("\n");
    pr_err!(
        "call stack: \n\t#1: {:#x} \n\t#0: {:#x} \n",
        regs.epc,
        regs.ra
    );
    pr_err!("tval: {:#x}\n", regs.tval);
    if regs.epc >= 4 {
        pr_err!("code: ");

        for addr in regs.epc - 4..regs.epc + 4 {
            let code = reg_read_a!(addr, u8);
            pr_err!("{:02x} ", code);
        }
    }

    pr_err!("\n");
    pr_err!("\n");
    pr_err!("{}", regs.context);
    pr_err!("\n");
    pr_err!("\n");
}

pub fn exception_handler(exception: Exception, regs: &Regs) {
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
        Exception::EnvironmentCallFromSMode => pr_err!("Environment call from S-mode."),
        Exception::EnvironmentCallFromMMode => pr_err!("Environment call from M-mode."),
        Exception::InstructionPageFault => pr_err!("Instruction page fault."),
        Exception::LoadPageFault => pr_err!("Load page fault."),
        Exception::StoreAmoPageFault => pr_err!("Store/AMO page fault."),
    }
    dump_stack(regs);
    loop {}
}
