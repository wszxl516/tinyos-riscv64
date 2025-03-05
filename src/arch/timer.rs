#![allow(dead_code)]
use crate::arch::trap::trap::Context;
use crate::config::{CLINT_BASE, ONE_TICK, TASK_SWITCH_INTERVAL_US};
use crate::pr_debug;
use crate::task::{set_task_ready_by_pid, task_switch};
use core::u64;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::ReadWrite;
#[derive(Debug, Clone, Copy)]
pub struct SoftTimer {
    us: u64,
    pid: usize,
}
impl SoftTimer {
    pub const fn empty() -> Self {
        Self { us: 0, pid: 0 }
    }
    pub const fn used(&self) -> bool {
        self.us != 0 && self.pid != 0
    }
}
pub struct Clint {
    base_addr: usize,
    one_ticks: u64,
    timers: [SoftTimer; 16],
}

impl Clint {
    const MTIME_OFFSET: usize = 0xbff8;
    const MTIME_CMP_OFFSET: usize = 0x4000;
    pub const fn new(addr: usize, tick: u64) -> Self {
        Self {
            base_addr: addr,
            one_ticks: tick,
            timers: [SoftTimer::empty(); 16],
        }
    }

    pub fn ticks(&self) -> u64 {
        let current = self.mtime_reg().get();
        current / self.one_ticks
    }

    fn msip_reg(&self, hartid: usize) -> &mut ReadWrite<u32> {
        unsafe { &mut *((self.base_addr + hartid * 4) as *mut ReadWrite<u32>) }
    }

    fn mtime_cmp_reg(&self, hartid: usize) -> &mut ReadWrite<u64> {
        unsafe {
            &mut *((self.base_addr + Self::MTIME_CMP_OFFSET + hartid * 8) as *mut ReadWrite<u64>)
        }
    }

    fn mtime_reg(&self) -> &mut ReadWrite<u64> {
        unsafe { &mut *((self.base_addr + Self::MTIME_OFFSET) as *mut ReadWrite<u64>) }
    }

    pub fn disable_timer(&self, hartid: usize) {
        self.mtime_cmp_reg(hartid).set(u64::MAX);
    }

    pub fn enable_timer(&mut self, hartid: usize) {
        let current = self.mtime_reg().get();
        self.mtime_cmp_reg(hartid).set(self.one_ticks + current);
    }

    pub fn irq_handler(&mut self, hartid: usize) {
        self.enable_timer(hartid);
    }
    pub fn set_soft_timer(&mut self, us: u64, pid: usize) -> u64 {
        let current = self.ticks();
        for t in &mut self.timers {
            if !t.used() {
                t.pid = pid;
                t.us = us + current;
                pr_debug!("set_timer: {:#x?}\n", t);
                break;
            }
        }
        current + us
    }
    pub fn complete_soft_timer(&mut self) {
        let current = self.ticks();
        for t in &mut self.timers {
            if t.used() && current >= t.us {
                pr_debug!("fetch_timer: {:#x?}\n", t);
                set_task_ready_by_pid(t.pid);
                *t = SoftTimer::empty();
            }
        }
    }
}

static mut CLINT: Clint = Clint::new(CLINT_BASE, ONE_TICK);

#[no_mangle]
pub fn setup_timer_m() {
    unsafe {
        CLINT.irq_handler(0);
    }
}

pub fn get_ticks() -> u64 {
    unsafe { CLINT.ticks() }
}

pub fn enable_timer_m() {
    unsafe {
        CLINT.enable_timer(0);
    }
}

pub fn setup_timer_s(stack_addr: &mut Context) {
    unsafe {
        CLINT.complete_soft_timer();
    }
    if get_ticks() % TASK_SWITCH_INTERVAL_US == 0 {
        task_switch(stack_addr)
    }
}

pub fn set_soft_timer(us: u64, pid: usize) -> u64 {
    unsafe { CLINT.set_soft_timer(us, pid) }
}
