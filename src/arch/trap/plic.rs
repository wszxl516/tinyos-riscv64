use super::super::super::config::PLIC_BASE;
use crate::{pr_err, reg_read_a, reg_write_a};

const NUM_IRQ: usize = 128;
//https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc
static mut PLIC: Plic = Plic {
    base_addr: 0,
    hart_id: 0,
    plic_handlers: [None; NUM_IRQ],
};

pub enum HandlerReturn {
    IntNoReschedule = 0,
    IntReschedule,
}

pub struct Plic {
    base_addr: usize,
    hart_id: usize,
    plic_handlers: [Option<fn()>; NUM_IRQ],
}

impl Plic {
    const CTRL_OFFSET: usize = 0x2080;
    const CTRL_CONTEXT_OFFSET: usize = 0x100;
    const PRIORITY_THRESHOLD: usize = 0x201000;
    const PRIORITY_THRESHOLD_INTERVAL: usize = 0x2000;
    const PLIC_CLAIM_OFFSET: usize = 0x201004;
    const PLIC_CLAIM_INTERVAL: usize = 0x2000;
    pub fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            hart_id: 0,
            plic_handlers: [None; NUM_IRQ],
        }
    }
    pub fn enable(&self, irq_num: usize) {
        assert!(irq_num <= NUM_IRQ);
        self.set_priority(irq_num as u32, 1);
        let ctrl_addr = self.base_addr
            + Self::CTRL_OFFSET
            + (Self::CTRL_CONTEXT_OFFSET * self.hart_id)
            + (4 * ((irq_num) / 32));
        let old = reg_read_a!(ctrl_addr, u32);
        reg_write_a!(ctrl_addr, !old | (1 << (irq_num % 32)), u32);
        self.set_priority_threshold(0)
    }
    fn set_priority_threshold(&self, priority: u32) {
        reg_write_a!(
            self.base_addr
                + Self::PRIORITY_THRESHOLD
                + (Self::PRIORITY_THRESHOLD_INTERVAL * self.hart_id),
            priority,
            u32
        );
    }
    fn set_priority(&self, irq_num: u32, value: u32) {
        reg_write_a!(self.base_addr + (4 * irq_num as usize), value, u32);
    }

    pub fn fetch_irq(&self) -> u32 {
        reg_read_a!(
            self.base_addr + Self::PLIC_CLAIM_OFFSET + (Self::PLIC_CLAIM_INTERVAL * self.hart_id),
            u32
        )
    }
    pub fn complete_irq(&self, irq_num: u32) {
        reg_write_a!(
            self.base_addr + Self::PLIC_CLAIM_OFFSET + (Self::PLIC_CLAIM_INTERVAL * self.hart_id),
            irq_num,
            u32
        )
    }
}

pub fn plic_init() {
    unsafe {
        PLIC.hart_id = 0;
        PLIC.base_addr = PLIC_BASE;
    }
}

pub fn register_handler(irq: usize, handler: fn()) {
    unsafe {
        PLIC.plic_handlers[irq] = Some(handler);
    }
    unsafe {
        PLIC.enable(irq);
    }
}

#[no_mangle]
pub fn platform_irq() -> HandlerReturn {
    let irq = unsafe { &PLIC }.fetch_irq();
    if irq == 0 {
        pr_err!("irq {:#x} IntNoReschedule !", irq);
        return HandlerReturn::IntNoReschedule;
    }
    let ret: HandlerReturn = HandlerReturn::IntNoReschedule;
    let handlers = unsafe { PLIC.plic_handlers.get(irq as usize) }.unwrap();
    match handlers {
        Some(handler) => handler(),
        None => pr_err!("can not found irq {:#x} handler!", irq),
    }
    unsafe { &PLIC }.complete_irq(irq);
    ret
}
