#![allow(dead_code)]
use crate::arch::timer::set_soft_timer;
use crate::arch::trap::trap::Context;
use crate::common::sleep::sleep_ms;
use crate::mm::config::PAGE_SIZE;
use crate::{pr_info, pr_notice};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
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
type TaskEntry = fn() -> !;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Ready,
    Running,
}
#[repr(C)]
#[derive(Debug, Clone)]
struct Task {
    pub context: Context,
    id: u32,
    pub name: &'static str,
    func: TaskEntry,
    stack: Stack<STACK_SIZE>,
    stack_size: usize,
    state: State,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}: [fn: {:?} stack: {}, stack size: {}]",
            self.name,
            self.func,
            self.stack.top(),
            self.stack_size
        )
    }
}

lazy_static! {
    static ref TASK_MGR: spin::Mutex<TaskManager> = spin::Mutex::new(TaskManager::new());
}
pub struct TaskManager {
    tasks: Vec<Task>,
    current: Option<usize>,
}

impl TaskManager {
    pub const fn new() -> TaskManager {
        Self {
            tasks: Vec::new(),
            current: None,
        }
    }
    pub fn add(&mut self, id: u32, name: &'static str, func: TaskEntry) {
        let stack = Stack::<STACK_SIZE>::new();
        let mut task = Task {
            context: Context::default(),
            id,
            name,
            func,
            stack_size: stack.size(),
            stack,
            state: State::Ready,
        };
        task.context.ra = task.func as usize;
        task.context.sp = task.stack.top();
        task.context.pc = task.func as usize;
        self.tasks.push(task);
    }
    #[no_mangle]
    pub fn switch(&mut self, regs: &mut Context) {
        if self.tasks.len() == 0 {
            return;
        }
        if let Some(index) = self.current {
            self.tasks[index].context.replace(regs);
            self.tasks[index].state = State::Ready;
        }
        let next = loop {
            let next = self.next();
            if next.state == State::Ready {
                // pr_info!("next: {}\n", next.name);
                break next;
            }
        };
        regs.replace(&next.context);
        next.state = State::Running;
    }
    fn current(&mut self) -> &mut Task {
        &mut self.tasks[*self.current.get_or_insert(0)]
    }
    fn next(&mut self) -> &mut Task {
        let next = match &mut self.current {
            None => *self.current.get_or_insert(0),
            Some(next) => {
                *next += 1;
                if *next >= self.tasks.len() {
                    *next = 0;
                };
                *next
            }
        };
        &mut self.tasks[next]
    }
}

pub fn init_task() {
    let mut mgr = TASK_MGR.lock();
    mgr.add(1, "demo0", demo0);
    mgr.add(2, "demo1", demo1);
    pr_notice!("{:-^50} \r\n", "");
    pr_notice!("finished task init!\n");
    pr_notice!("{:-^50} \r\n", "");
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
        // unsafe { core::arch::asm!("ld t0, 0({tmp})", tmp = in(reg) usize::MAX) }
        set_soft_timer(1000 * 1000, 0);
    }
}
#[no_mangle]
pub fn task_switch(regs: &mut Context) {
    TASK_MGR.lock().switch(regs)
}
