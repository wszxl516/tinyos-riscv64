pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: usize = 12;
pub const PTE_SHIFT: usize = 10;
pub const PPN_BITS: usize = 9;
pub const SATP_SV39: usize = 8 << 60;
pub const PPN_MASK: usize = 0x1ff;
#[macro_export]
macro_rules! align_up {
    ($address: expr) => {
        ((($address) + crate::mm::config::PAGE_SIZE - 1) & !(crate::mm::config::PAGE_SIZE - 1))
    };
}

#[macro_export]
macro_rules! align_down {
    ($address: expr) => {
        (($address) & !(crate::mm::config::PAGE_SIZE - 1))
    };
}

#[macro_export]
macro_rules! mem_set {
    ($address: expr, $len: expr, $value: expr) => {
            unsafe { core::slice::from_raw_parts_mut($address, $len).fill($value) }
    };
}