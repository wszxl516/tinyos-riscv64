use crate::mm::address::PhyAddr;
use crate::mm::config;
use crate::mm::config::PPN_MASK;
use crate::mm::page::PageTable;
use bitflags::bitflags;
use core::fmt::{Display, Formatter};

//https://five-embeddev.com/riscv-isa-manual/latest/supervisor.html
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Entry(usize);
bitflags! {
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
        const RW = 1 << 1 | 1 << 2;
        const RWX = 1 << 1 | 1 << 2 | 1 << 3;
    }
}
impl PTEFlags {
    const FLAG_STR: [char; 8] = ['V', 'R', 'W', 'X', 'U', 'G', 'A', 'D'];
}

impl Display for PTEFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for i in (0usize..8).rev() {
            if self.contains(Self::from_bits(1 << i).unwrap()) {
                write!(f, "{}", PTEFlags::FLAG_STR[i]).unwrap()
            } else {
                write!(f, "-").unwrap()
            }
        }
        Ok(())
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let ppn1 = self.get_ppn(0);
        let ppn2 = self.get_ppn(1);
        let ppn3 = self.get_ppn(2);

        write!(f, "{:#x}|{:#x}|{:#x}|{}", ppn1, ppn2, ppn3, self.flags())
    }
}

impl Entry {
    pub fn get_ppn(&self, index: usize) -> usize {
        assert_eq!(index <= 2, true);
        self.0 >> config::PTE_SHIFT >> ((index - 1) * 9) & (PPN_MASK)
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits((self.0 & 0xff) as u8).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    pub fn get_page_table(&self) -> &'static mut PageTable {
        PhyAddr::new(((self.0) >> config::PTE_SHIFT) << config::PAGE_SHIFT).as_table()
    }
    pub fn get_phy_addr(&self) -> PhyAddr {
        PhyAddr::new(((self.0) >> 10) << 12)
    }
    pub fn set_value(&mut self, value: usize) {
        self.0 = value
    }
}
