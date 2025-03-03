//https://chromite.readthedocs.io/en/latest/mmu.html
pub mod address;
pub mod config;
pub mod entry;
pub mod page;

#[link_section = ".root_table"]
static mut ROOT_PAGE: PageTable = PageTable::empty();

use crate::config::MEM_SIZE;
use crate::mm::address::{PhyAddr, VirtAddr};
use crate::mm::config::{PAGE_SHIFT, PAGE_SIZE};
use crate::mm::entry::PTEFlags;
use crate::mm::page::PageTable;
use crate::{lds_address, mem_set, pr_notice, reg_write_p};
use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
#[allow(dead_code)]
pub fn page_alloc(pages: usize) -> usize {
    let addr =
        unsafe { ALLOCATOR.alloc(Layout::from_size_align(pages * PAGE_SIZE, PAGE_SIZE).unwrap()) };
    mem_set!(addr, pages * PAGE_SIZE, 0);
    addr.addr()
}

#[allow(dead_code)]
pub fn page_free(start: usize, pages: usize) {
    unsafe {
        ALLOCATOR.dealloc(
            start as *mut u8,
            Layout::from_size_align(pages * PAGE_SIZE, PAGE_SIZE).unwrap(),
        )
    }
}
pub fn init_heap() {
    let mem_size = MEM_SIZE - (lds_address!(heap_start) - lds_address!(base_addr));
    let heap_start = lds_address!(heap_start) as *mut u8;
    pr_notice!("{:-^50}\n", "");
    pr_notice!("{: ^50} \r\n", "Heap init");
    pr_notice!("{:-^50}\n", "");
    pr_notice!(
        "{: ^10} [0x{:<10x}  0x{:x}]\n",
        "",
        heap_start as usize,
        mem_size
    );
    unsafe {
        ALLOCATOR.lock().init(heap_start, mem_size);
    }
}

pub fn map(va: VirtAddr, pa: PhyAddr, size: usize, flags: PTEFlags, name: &str) {
    pr_notice!("{:-^50} \r\n", "");
    pr_notice!(
        "{: <10}  [0x{:<10x} - 0x{:<10x} {}]\n\r",
        name,
        va.0,
        size,
        flags
    );
    unsafe {
        ROOT_PAGE.map(va, pa, size, flags);
    }
}

#[inline]
pub fn flush_tlb() {
    unsafe { asm!("sfence.vma zero, zero") }
}

pub fn enable_mmu(root: PhyAddr) {
    reg_write_p!(satp, config::SATP_SV39 | root.0 >> PAGE_SHIFT);
    flush_tlb()
}

pub fn setup_mmu() {
    pr_notice!("{:-^50} \r\n", "");
    pr_notice!("{: ^50} \r\n", "Memory Map");
    //uart
    let va = VirtAddr::new(crate::config::PCI_CONFIG_START);
    let pa = PhyAddr::new(crate::config::PCI_CONFIG_START);
    map(va, pa, PAGE_SIZE * 32, PTEFlags::RW, "pci config");
    //uart
    let va = VirtAddr::new(crate::config::PCI_MEM_START);
    let pa = PhyAddr::new(crate::config::PCI_MEM_START);
    map(va, pa, PAGE_SIZE * 16, PTEFlags::RW, "pci memory");

    //uart
    let va = VirtAddr::new(crate::config::BASE_UART);
    let pa = PhyAddr::new(crate::config::BASE_UART);
    map(va, pa, PAGE_SIZE, PTEFlags::RW, "uart");
    //rtc
    let va = VirtAddr::new(crate::config::RTC_BASE_ADDR);
    let pa = PhyAddr::new(crate::config::RTC_BASE_ADDR);
    map(va, pa, PAGE_SIZE, PTEFlags::RW, "rtc");
    //clint
    let va = VirtAddr::new(crate::config::CLINT_BASE);
    let pa = PhyAddr::new(crate::config::CLINT_BASE);
    map(va, pa, PAGE_SIZE * 16, PTEFlags::RW, "clint");
    //plic
    let va = VirtAddr::new(crate::config::PLIC_BASE);
    let pa = PhyAddr::new(crate::config::PLIC_BASE);
    map(va, pa, 0x600000, PTEFlags::RW, "plic");
    //text
    let size = lds_address!(text_end) - lds_address!(text_start);
    let va = VirtAddr::new(lds_address!(text_start));
    let pa = PhyAddr::new(lds_address!(text_start));
    map(va, pa, size, PTEFlags::RWX, "text");
    //bss
    let size = lds_address!(bss_end) - lds_address!(bss_start);
    let va = VirtAddr::new(lds_address!(bss_start));
    let pa = PhyAddr::new(lds_address!(bss_start));
    map(va, pa, size, PTEFlags::RW, "bss");
    //rodata
    let size = lds_address!(ro_end) - lds_address!(ro_start);
    let va = VirtAddr::new(lds_address!(ro_start));
    let pa = PhyAddr::new(lds_address!(ro_start));
    map(va, pa, size, PTEFlags::R, "rodata");
    //data
    let size = lds_address!(data_end) - lds_address!(data_start);
    let va = VirtAddr::new(lds_address!(data_start));
    let pa = PhyAddr::new(lds_address!(data_start));
    map(va, pa, size, PTEFlags::RW, "data");
    //stack
    let size = lds_address!(stack_top) - lds_address!(stack_bottom);
    let va = VirtAddr::new(lds_address!(stack_bottom));
    let pa = PhyAddr::new(lds_address!(stack_bottom));
    map(va, pa, size, PTEFlags::RW, "stack");
    //symbols
    let size = lds_address!(symbols_end) - lds_address!(symbols_start);
    let va = VirtAddr::new(lds_address!(symbols_start));
    let pa = PhyAddr::new(lds_address!(symbols_start));
    map(va, pa, size, PTEFlags::RW, "symbols");
    //heap
    let va = VirtAddr::new(lds_address!(heap_start));
    let pa = PhyAddr::new(lds_address!(heap_start));
    map(
        va,
        pa,
        MEM_SIZE - (lds_address!(heap_start) - lds_address!(base_addr)),
        PTEFlags::RW,
        "heap",
    );
    pr_notice!("{:-^50} \r\n", "");
    pr_notice!("{:-^50} \r\n", "");
    enable_mmu(unsafe { ROOT_PAGE.addr() });
}
