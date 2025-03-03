#![allow(dead_code)]

use crate::arch::plic::platform_irq;
use crate::arch::timer::setup_timer_s;
use crate::arch::trap::trap::Context;
use crate::impl_numeric_enum;
use crate::{pr_err, reg_clear_bit_p};

impl_numeric_enum! {
    u32,
    pub Interrupt [
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
    ]
}

pub fn interrupt_handler(interrupt: Interrupt, stack_addr: &mut Context) {
    match interrupt {
        Interrupt::SupervisorSoftwareInterrupt => {
            reg_clear_bit_p!(sip, 1 << 1);
            setup_timer_s(stack_addr);
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
        Interrupt::Unknown(v) => {
            panic!("Unknown Interrupt: {:#x}\n", v)
        }
    }
}
