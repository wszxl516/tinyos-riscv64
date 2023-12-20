use crate::config::FRAME_SIZE;
use crate::mm::config::PAGE_SIZE;

static mut FRAME_ALLOCATOR: FrameAllocator<256> = FrameAllocator::new(0, 0);

#[derive(Debug)]
pub struct FrameAllocator<const SIZE: usize> {
    current: usize,
    end: usize,
    recycled: [usize; SIZE],
    recycled_index: usize,
}

impl<const SIZE: usize> FrameAllocator<SIZE> {
    pub const fn new(start: usize, end: usize) -> Self {
        Self {
            current: start,
            end,
            recycled: [0; SIZE],
            recycled_index: 0,
        }
    }
    pub fn init(&mut self, start: usize, end: usize) {
        self.current = start;
        self.end = end;
    }
    fn alloc(&mut self) -> Option<usize> {
        if self.recycled_index == 0 {
            if self.current == self.end {
                None
            } else {
                self.current += PAGE_SIZE;
                Some(self.current - PAGE_SIZE)
            }
        } else {
            Some(self.recycled[self.recycled_index - 1])
        }
    }
    #[allow(dead_code)]
    fn dealloc(&mut self, addr: usize) {
        if addr >= self.current || self.recycled.iter().find(|&v| *v == addr).is_some() {
            panic!("Frame addr={:#x} has not been allocated!", addr);
        }
        // recycle
        self.recycled[self.recycled_index] = addr;
        self.recycled_index += 1
    }
}

pub fn init_frame_allocator() {
    unsafe {
        FRAME_ALLOCATOR.init(
            *super::ld_script_addr::FRAME_START,
            *super::ld_script_addr::FRAME_START + FRAME_SIZE,
        );
    }
}

pub fn alloc_page() -> Option<usize> {
    unsafe {
        match FRAME_ALLOCATOR.alloc() {
            None => None,
            Some(addr) => {
                mem_set(addr, PAGE_SIZE, 0);
                Some(addr)
            }
        }
    }
}

#[allow(dead_code)]
pub fn dealloc_page(addr: usize) {
    unsafe { FRAME_ALLOCATOR.dealloc(addr) }
}

pub fn mem_set(addr: usize, size: usize, value: u8) {
    let addr = addr as *mut u8;
    for i in 0..size {
        unsafe { addr.add(i).write(value) }
    }
}
