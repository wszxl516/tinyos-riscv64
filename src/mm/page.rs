use super::address::{PhyAddr, VirtAddr};
use super::entry::{Entry, PTEFlags};
use crate::align_down;
use crate::mm::config::PAGE_SIZE;
use crate::mm::{page_alloc};

#[repr(C)]
#[derive(Debug)]
pub struct PageTable {
    pub root: [Entry; 512],
}

impl PageTable {
    pub const fn from_address(addr: usize) -> *mut Self {
        addr as *mut Self
    }
    pub fn map(&mut self, va: VirtAddr, pa: PhyAddr, size: usize, flags: PTEFlags) {
        let mut va_start = VirtAddr::new(align_down!(va.0));
        let va_end = VirtAddr::new(align_down!(va.0 + size - 1));
        let mut phy_start = pa;
        loop {
            let pte = self.find_entry(va_start);
            unsafe {
                match (*pte).is_valid() {
                    true => {
                        panic!("remap {}!", (*pte).get_phy_addr().0)
                    }
                    false => {
                        (*pte).set_value(phy_start.pte_value(flags | PTEFlags::V));
                    }
                }
            }
            if va_start == va_end {
                break;
            }
            va_start += VirtAddr::new(PAGE_SIZE);
            phy_start += PhyAddr::new(PAGE_SIZE);
        }
    }
    #[allow(dead_code)]
    pub fn unmap(&mut self, va: VirtAddr, size: usize) {
        let mut va_start = VirtAddr::new(align_down!(va.0));
        let va_end = VirtAddr::new(align_down!(va.0 + size - 1));
        loop {
            let pte = self.find_entry(va_start);
            unsafe {
                match (*pte).is_valid() {
                    true => {
                        (*pte).set_value(0);
                    }
                    false => {
                        panic!("is not mapped {:#x}!", va_start.0)
                    }
                }
            }
            if va_start == va_end {
                break;
            }
            va_start += VirtAddr::new(PAGE_SIZE);
        }
    }
    pub fn addr(&self) -> PhyAddr {
        PhyAddr::new(core::ptr::addr_of!(self.root).addr())
    }
    pub fn find_entry(&mut self, va: VirtAddr) -> *mut Entry {
        let mut page_table = self;
        let mut entry;
        for level in [2, 1] {
            entry = &mut page_table.root[va.get_ppn(level)];
            if entry.is_valid() {
                page_table = entry.get_page_table();
            } else {
                let addr = page_alloc(1);
                unsafe {
                    page_table = &mut *(addr as *mut PageTable);
                }
                entry.set_value(page_table.addr().pte_value(PTEFlags::V));
            }
        }
        &mut (page_table.root[va.get_ppn(0)])
    }
}
