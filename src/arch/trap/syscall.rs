use super::trap::Regs;
use crate::arch::timer::set_soft_timer;
use crate::task::task_switch;
use crate::task::{current_pid, set_state_with_pid, State};
use crate::{impl_numeric_enum, pr_warn};

impl_numeric_enum! {
    usize,
    #[derive(Debug, Clone, Copy)]
    pub SyscallNo [
        Sleep = 1
    ]
}

pub fn syscall(regs: &mut Regs) {
    match SyscallNo::from_value(regs.context.a7) {
        SyscallNo::Sleep => {
            {
                let pid = current_pid();
                set_state_with_pid(pid, State::Sleeping);
                set_soft_timer(regs.context.a0 as u64, pid);
            }
            task_switch(&mut regs.context);
            regs.context.a0 = 0
        }
        SyscallNo::Unknown(no) => {
            pr_warn!("Unknown syscall no: {:#x}\n", no);
            regs.context.a0 = 1
        }
    }
}
