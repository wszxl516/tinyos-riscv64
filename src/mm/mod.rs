pub mod address;
pub mod config;
pub mod entry;
pub mod page;

static mut ROOT_PAGE: Option<*mut PageTable> = None;

use core::alloc::{GlobalAlloc, Layout};
use crate::config::MEM_SIZE;
use crate::mm::address::{PhyAddr, VirtAddr};
use crate::mm::config::{PAGE_SHIFT, PAGE_SIZE};
use crate::mm::entry::PTEFlags;
use crate::mm::page::PageTable;
use crate::{mem_set, pr_notice, reg_write_p};
use core::arch::asm;
use linked_list_allocator::LockedHeap;

pub mod ld_script_addr {
    use lazy_static::lazy_static;

    extern "C" {
        fn heap_start();
        fn stack_top();
        fn stack_bottom();
        fn bss_start();
        fn global_pointer();
        fn text_start();
        fn text_end();
        fn base_addr();
        fn frame_start();
        fn ro_start();
        fn data_start();
    }
    lazy_static! {
        pub static ref HEAP_START: usize = heap_start as usize;
        pub static ref STACK_TOP: usize = stack_top as usize;
        pub static ref STACK_BOTTOM: usize = stack_bottom as usize;
        pub static ref BSS_START: usize = bss_start as usize;
        pub static ref GLOBAL_POINTER: usize = global_pointer as usize;
        pub static ref TEXT_START: usize = text_start as usize;
        pub static ref TEXT_END: usize = text_end as usize;
        pub static ref BASE_ADDDR: usize = base_addr as usize;
        pub static ref FRAME_START: usize = frame_start as usize;
        pub static ref RO_START: usize = ro_start as usize;
        pub static ref DATA_START: usize = data_start as usize;
        pub static ref KERNEL_SIZE: usize = heap_start as usize - base_addr as usize;
    }
}

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
#[allow(dead_code)]
pub fn page_alloc(pages: usize) -> usize {
    let addr = unsafe {
        ALLOCATOR.alloc(Layout::from_size_align(pages * PAGE_SIZE, PAGE_SIZE).unwrap())
    };
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
    let mem_size = MEM_SIZE - (*ld_script_addr::KERNEL_SIZE);
    let heap_start = (*ld_script_addr::HEAP_START) as *mut u8;
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
        match ROOT_PAGE {
            None => panic!("ROOT_PAGE not exists!"),
            Some(root) => {
                (*root).map(va, pa, size, flags);
            }
        }
    }
}

#[inline]
pub fn flush_tlb() {
    unsafe { asm!("sfence.vma zero, zero") }
}

pub fn enable_mmu(root: *mut PageTable) {
    reg_write_p!(satp, config::SATP_SV39 | (*root).addr().0 >> PAGE_SHIFT);
    flush_tlb()
}

pub fn setup_mmu() {
    unsafe {
        let root_page = PageTable::from_address(page_alloc(1));
        ROOT_PAGE = Some(root_page);
    }
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
    map(va, pa, PAGE_SIZE, PTEFlags::RW, "clint");
    //plic
    let va = VirtAddr::new(crate::config::PLIC_BASE);
    let pa = PhyAddr::new(crate::config::PLIC_BASE);
    map(va, pa, 0x600000, PTEFlags::RW, "plic");
    //text
    let size = *ld_script_addr::BSS_START - *ld_script_addr::TEXT_START;
    let va = VirtAddr::new(*ld_script_addr::TEXT_START);
    let pa = PhyAddr::new(*ld_script_addr::TEXT_START);
    map(va, pa, size, PTEFlags::RWX, "text");
    //bss
    let size = *ld_script_addr::RO_START - *ld_script_addr::BSS_START;
    let va = VirtAddr::new(*ld_script_addr::BSS_START);
    let pa = PhyAddr::new(*ld_script_addr::BSS_START);
    map(va, pa, size, PTEFlags::RW, "bss");
    //rodata
    let size = *ld_script_addr::DATA_START - *ld_script_addr::RO_START;
    let va = VirtAddr::new(*ld_script_addr::RO_START);
    let pa = PhyAddr::new(*ld_script_addr::RO_START);
    map(va, pa, size, PTEFlags::R, "rodata");
    //data
    let size = *ld_script_addr::STACK_BOTTOM - *ld_script_addr::DATA_START;
    let va = VirtAddr::new(*ld_script_addr::DATA_START);
    let pa = PhyAddr::new(*ld_script_addr::DATA_START);
    map(va, pa, size, PTEFlags::RW, "data");
    //stack
    let size = *ld_script_addr::STACK_TOP - *ld_script_addr::STACK_BOTTOM;
    let va = VirtAddr::new(*ld_script_addr::STACK_BOTTOM);
    let pa = PhyAddr::new(*ld_script_addr::STACK_BOTTOM);
    map(va, pa, size, PTEFlags::RW, "stack");
    //heap
    let va = VirtAddr::new(*ld_script_addr::FRAME_START);
    let pa = PhyAddr::new(*ld_script_addr::FRAME_START);
    map(va, pa, MEM_SIZE, PTEFlags::RW, "heap");
    pr_notice!("{:-^50} \r\n", "");
    pr_notice!("{:-^50} \r\n", "");
    enable_mmu(unsafe { ROOT_PAGE }.unwrap());
}
