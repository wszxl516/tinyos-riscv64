#![allow(dead_code)]

use crate::arch::timer::get_ticks;
use crate::arch::trap::plic::platform_irq;
use crate::arch::trap::trap::Context;
use crate::task::task_switch;
use crate::{pr_err, reg_clear_bit_p};

#[repr(u32)]
pub enum Interrupt {
    //1 0 RESERVED
    SupervisorSoftwareInterrupt = 1,
    //1 2 RESERVED
    MachineSoftwareInterrupt = 3,
    //1 4 RESERVED
    SupervisorTimerInterrupt = 5,
    //1 6 RESERVED
    MachineTimerInterrupt = 7,
    //1 8 RESERVED
    SupervisorExternalInterrupt = 9,
    //1 10 RESERVED
    MachineExternalInterrupt = 11,
    //1 12–15 RESERVED
    // 1 ≥16 DESIGNATED FOR PLATFORM USE
}

impl Interrupt {
    pub fn from_u32(value: u32) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

pub fn interrupt_handler(interrupt: Interrupt, stack_addr: &mut Context) {
    match interrupt {
        Interrupt::SupervisorSoftwareInterrupt => {
            reg_clear_bit_p!(sip, 1 << 1);
            if get_ticks() % 500 == 0 {
                task_switch(stack_addr)
            }
        }
        Interrupt::SupervisorExternalInterrupt => {
            platform_irq();
        }
        Interrupt::SupervisorTimerInterrupt => {
            pr_err!("Supervisor timer Interrupt.");
        }
        Interrupt::MachineSoftwareInterrupt => unreachable!("Machine software Interrupt."),
        Interrupt::MachineTimerInterrupt => unreachable!("Machine timer Interrupt."),
        Interrupt::MachineExternalInterrupt => unreachable!("Machine External Interrupt."),
    }
}
