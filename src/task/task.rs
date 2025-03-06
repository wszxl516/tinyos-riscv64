#![allow(dead_code)]
use super::queue::TaskQueue;
use crate::arch::trap::trap::Context;
use crate::config::TASK_SWITCH_INTERVAL_US;
use crate::impl_numeric_enum;
use crate::mm::config::PAGE_SIZE;
use alloc::boxed::Box;
use core::cmp::{max, min};
use core::fmt::{Display, Formatter, Result};
use core::ops::{Deref, DerefMut};
use lazy_static::lazy_static;
const STACK_SIZE: usize = PAGE_SIZE;

#[derive(Debug, Clone)]
#[repr(C, align(16))]
pub struct Stack<const SIZE: usize>(Box<[u8; SIZE]>);
impl<const SIZE: usize> Stack<SIZE> {
    pub fn new() -> Self {
        Self(Box::new([0u8; SIZE]))
    }
    pub fn top(&self) -> usize {
        self.0.as_ptr().addr() + SIZE
    }
    pub fn size(&self) -> usize {
        SIZE
    }
}
impl<const SIZE: usize> Display for Stack<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Stack {{ addr: {:#x}, size: {:#x} }}",
            self.0.as_ptr().addr(),
            SIZE
        )
    }
}
type TaskEntry = fn() -> !;
impl_numeric_enum! {
    u8,
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub State [
        Sleeping = 1,
        Ready = 2,
        Running = 3,
        Exited = 4,
    ]
}
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Self::Sleeping => "Sleeping",
            Self::Ready => "Ready",
            Self::Running => "Running",
            Self::Exited => "Exited",
            Self::Unknown(_) => "Unknown",
        };
        write!(f, "{}", s)
    }
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Task {
    pub context: Context,
    pub id: usize,
    pub name: &'static str,
    func: TaskEntry,
    stack: Stack<STACK_SIZE>,
    pub state: State,
    pub priority: u8,
    pub time_slice: u8,
    pub total_time: u64,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Task {{ id: {}, entry: {:#x}, {} priority: {}, time_slice: {}}}",
            self.id, self.func as *const u8 as usize, self.stack, self.priority, self.time_slice
        )
    }
}

lazy_static! {
    static ref TASK_MGR: spin::RwLock<TaskManager> = spin::RwLock::new(TaskManager::new());
}
pub struct TaskManager {
    tasks: TaskQueue<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        let mut mgr = Self {
            tasks: TaskQueue::new(),
        };
        let stack = Stack::<STACK_SIZE>::new();
        let mut task = Task {
            context: Context::empty(),
            id: 0,
            name: "idle",
            func: Self::idle_task,
            stack,
            state: State::Ready,
            priority: 1,
            time_slice: 1,
            total_time: 0,
        };
        task.context.ra = task.func as usize;
        task.context.sp = task.stack.top();
        task.context.pc = task.func as usize;
        mgr.tasks.add(task);
        mgr
    }
    fn idle_task() -> ! {
        loop {
            unsafe {
                core::arch::riscv64::wfi();
            }
        }
    }
    pub fn add(&mut self, id: usize, name: &'static str, func: TaskEntry, priority: u8) {
        assert!(id != 0);
        let stack = Stack::<STACK_SIZE>::new();
        let mut task = Task {
            context: Context::empty(),
            id,
            name,
            func,
            stack,
            state: State::Ready,
            priority,
            time_slice: priority,
            total_time: 0,
        };
        task.context.ra = task.func as usize;
        task.context.sp = task.stack.top();
        task.context.pc = task.func as usize;
        self.tasks.add(task);
    }
    #[no_mangle]
    pub fn switch(&mut self, regs: &mut Context) {
        if self.tasks.len() == 1 {
            return;
        }
        //Weighted Round-Robin Scheduling
        if let Some(current) = self.tasks.current_mut() {
            if current.time_slice == 0 || current.state == State::Sleeping {
                current.context.replace(regs);
                if current.state == State::Running {
                    current.state = State::Ready;
                }
                if current.state != State::Sleeping {
                    current.total_time += current.priority as u64;
                    current.time_slice = current.priority;
                } else {
                    //When a task is sleeping, reduce the time slice of the task
                    if current.time_slice != 0 {
                        let used_slice = max(current.priority - current.time_slice, 1);
                        current.total_time += used_slice as u64;
                        if current.time_slice <= used_slice {
                            current.time_slice = 1
                        } else {
                            current.time_slice -= used_slice;
                        }
                    } else {
                        current.total_time += current.priority as u64;
                        current.time_slice = current.priority;
                    }
                }
            } else {
                current.time_slice -= 1;
                return;
            }
        }

        let next = loop {
            let next = self.tasks.next();
            if next.state == State::Ready {
                break next;
            }
        };
        regs.replace(&next.context);
        next.state = State::Running;
    }
    fn set_state_by_pid(&mut self, pid: usize, state: State) {
        for task in self.tasks.deref_mut() {
            if task.id == pid {
                task.state = state.clone();
            }
        }
    }
    fn wakeup_by_pid(&mut self, pid: usize, sleep_time: u64) {
        for task in self.tasks.deref_mut() {
            if task.id == pid {
                task.state = State::Ready;
                let sleep_slice = min(
                    (sleep_time / TASK_SWITCH_INTERVAL_US / 10) as u8,
                    max(task.priority / 4, 1),
                );
                // When the task is wakeup, increase the time slice
                task.time_slice += min(
                    sleep_slice,
                    task.priority.overflowing_sub(task.time_slice).0,
                )
            }
        }
    }
    fn current_pid(&self) -> Option<usize> {
        self.tasks.current().map(|x| x.id)
    }
}

#[inline]
pub fn each_task<F: FnMut(&Task)>(mut f: F) {
    for task in TASK_MGR.read().tasks.deref() {
        f(task)
    }
}
#[inline]
pub fn task_add(id: usize, name: &'static str, func: TaskEntry, priority: u8) {
    TASK_MGR.write().add(id, name, func, priority);
}
#[inline]
pub fn current_task_pid() -> Option<usize> {
    TASK_MGR.read().current_pid()
}
#[inline]
pub fn set_task_state_by_pid(pid: usize, state: State) {
    TASK_MGR.write().set_state_by_pid(pid, state);
}
#[inline]
pub fn wakeup_by_pid(pid: usize, sleep_time: u64) {
    TASK_MGR.write().wakeup_by_pid(pid, sleep_time)
}
#[no_mangle]
pub fn task_switch(regs: &mut Context) {
    TASK_MGR.write().switch(regs)
}
