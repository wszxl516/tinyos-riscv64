use super::super::config::SMP_COUNT;
use crate::{pr_notice, reg_read_p};

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Feature {
    hart_id: u32,
    bit: u16,
    extensions: u32,
    vendor_id: u16,
    machine_id: u16,
    impl_id: u16,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct CPU {
    feature: Feature,
    online: bool,
    start_fn: fn() -> !,
}

static mut CPUS: [CPU; SMP_COUNT] = [CPU {
    feature: Feature {
        hart_id: 0,
        bit: 0,
        extensions: 0,
        vendor_id: 0,
        machine_id: 0,
        impl_id: 0,
    },
    online: false,
    start_fn: || loop {
        unsafe { core::arch::riscv64::wfi() }
    },
}; SMP_COUNT];

impl CPU {
    pub fn fetch(&mut self) {
        self.feature = Feature {
            hart_id: reg_read_p!(mhartid) as u32,
            bit: match reg_read_p!(misa) >> 62 {
                1 => 32,
                2 => 64,
                3 => 128,
                _ => 0,
            },
            extensions: reg_read_p!(misa) as u32,
            vendor_id: reg_read_p!(mvendorid) as u16,
            machine_id: reg_read_p!(marchid) as u16,
            impl_id: reg_read_p!(mimpid) as u16,
        };
        self.online = true;
    }
    #[allow(dead_code)]
    pub fn new_empty() -> Self {
        Self {
            feature: Feature {
                hart_id: 0,
                bit: 0,
                extensions: 0,
                vendor_id: 0,
                machine_id: 0,
                impl_id: 0,
            },
            online: false,
            start_fn: || loop {
                unsafe { core::arch::riscv64::wfi() }
            },
        }
    }
    pub fn dump(&self) {
        let mut buffer = [0u8; 26];
        for i in 0..26 {
            //reserved
            if i == 7 || i == 10 || i == 14 || i == 17 || i == 22 || i == 24 || i == 25 {
                continue;
            }
            if ((self.feature.extensions & (1 << i)) >> i).eq(&1) {
                buffer[i] = b'A'.to_ascii_uppercase() + i as u8;
            } else {
                buffer[i] = 1;
            }
        }
        pr_notice!("{:-^50} \n", "");
        pr_notice!(
            "Core: {}, Arch: Riscv{}, \nFlags: {} \n",
            self.feature.hart_id,
            self.feature.bit,
            unsafe { core::str::from_utf8_unchecked(&buffer) }
        );
        pr_notice!(
            "Vendor ID: {:#x}.{:#x}.{:#x}\n",
            self.feature.vendor_id,
            self.feature.machine_id,
            self.feature.impl_id
        );
    }
}

pub fn init() {
    let cpu_id = reg_read_p!(mhartid);
    unsafe {
        CPUS[cpu_id].fetch();
    }
    if cpu_id.ne(&0) {
        unsafe {
            (CPUS[cpu_id].start_fn)();
        }
    }
}

pub fn dump_features() {
    unsafe {
        for i in CPUS.iter() {
            i.dump()
        }
    }
}
