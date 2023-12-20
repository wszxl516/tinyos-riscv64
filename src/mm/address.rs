use super::entry::PTEFlags;
use crate::mm::config::{PAGE_SHIFT, PPN_BITS, PPN_MASK, PTE_SHIFT};
use crate::mm::page::PageTable;
use core::ops::AddAssign;

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhyAddr(pub usize);

impl AddAssign for VirtAddr {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl VirtAddr {
    pub fn new(addr: usize) -> Self {
        Self { 0: addr }
    }
    pub fn get_ppn(&self, level: usize) -> usize {
        assert_eq!(level <= 2, true);
        ((self.0) >> (PAGE_SHIFT + (PPN_BITS * level))) & PPN_MASK
    }
}

impl AddAssign for PhyAddr {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl PhyAddr {
    pub fn new(addr: usize) -> Self {
        Self { 0: addr }
    }

    pub fn as_table(&self) -> &'static mut PageTable {
        unsafe { &mut *(self.0 as *mut PageTable) }
    }

    pub fn pte_value(&self, p: PTEFlags) -> usize {
        (self.0 >> PAGE_SHIFT << PTE_SHIFT) | p.bits() as usize
    }
}
