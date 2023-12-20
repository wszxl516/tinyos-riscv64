use crate::arch::trap::trap::Context;
use crate::mm::config::PAGE_SIZE;
use crate::{pr_info, pr_notice};
use alloc::boxed::Box;
use core::arch::asm;
use core::fmt::{Display, Formatter};

const STACK_SIZE: usize = PAGE_SIZE / 4;

type TaskFn = fn() -> !;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Task {
    pub context: Context,
    id: u32,
    pub name: &'static str,
    func: TaskFn,
    stack: usize,
    stack_size: usize,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}: [fn: {:?} stack: {}, stack size: {}]",
            self.name, self.func, self.stack, self.stack_size
        )
    }
}

static mut TASK_MGR: TaskManager<3> = TaskManager::new();

pub struct TaskManager<const MAX_TASK: usize> {
    tasks: [Option<Task>; MAX_TASK],
    tasks_index: usize,
    tasks_len: usize,
}

impl<const MAX_TASK: usize> TaskManager<MAX_TASK> {
    pub const fn new() -> TaskManager<MAX_TASK> {
        Self {
            tasks: [None; MAX_TASK],
            tasks_index: 0,
            tasks_len: 0,
        }
    }
    pub fn add(&mut self, id: u32, name: &'static str, func: TaskFn) {
        let mut task = Task {
            context: Context::empty_new(),
            id,
            name,
            func,
            stack: 0,
            stack_size: 0,
        };
        let stack = Box::leak(Box::new([0u8; STACK_SIZE]));
        task.context.ra = task.func as usize;
        task.context.sp = stack.as_ptr() as usize + STACK_SIZE;
        task.context.pc = task.func as usize;
        task.stack = stack.as_ptr() as usize;
        task.stack = STACK_SIZE;
        self.tasks[self.tasks_len] = Some(task);
        self.tasks_len += 1
    }
    pub fn switch(&mut self, regs: &mut Context) {
        if self.tasks_index == 0 {} else {
            let mut t = self.tasks[self.tasks_index].unwrap();
            t.context.replace(regs);
            self.tasks[self.tasks_index].replace(t);
        }
        self.tasks_index += 1;
        if self.tasks_index == self.tasks_len {
            self.tasks_index = 0
        }
        regs.replace(&self.tasks[self.tasks_index].unwrap().context);
    }
}

pub fn init_task() {
    unsafe {
        TASK_MGR.add(0, "idle", || loop {
            asm!("wfi")
        });
        TASK_MGR.add(1, "test", || loop {
            pr_info!("test\n");
            asm!("wfi")
        });
        TASK_MGR.add(2, "idle", || loop {
            for x in 0..10 {
                for y in 0..10 {
                    pr_info!("demo{} - {}\n", x, y);
                    asm!("wfi")
                }
            }
        });
    }
    pr_notice!("{:-^50} \r\n", "");
    pr_notice!("finished task init!\n");
    pr_notice!("{:-^50} \r\n", "");
}

#[no_mangle]
pub fn task_switch(regs: &mut Context) {
    unsafe { TASK_MGR.switch(regs) }
}
