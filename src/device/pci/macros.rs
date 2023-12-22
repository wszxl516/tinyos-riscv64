#[macro_export]
macro_rules! fdt_get_u64 {
    ($value: expr) => {
        {
            let (b, r): (&[u8;8], &[u8]) = $value.split_first_chunk().unwrap();
            (r, u64::from_be_bytes(*b))
        }
    };
}
#[macro_export]
macro_rules! fdt_get_u32 {
    ($value: expr) => {
        {
            let (b, r):(&[u8;4], &[u8]) = $value.split_first_chunk().unwrap();
            (r, u32::from_be_bytes(*b))
        }
    };
}

#[macro_export]
macro_rules! pci_addr {
    ($base: expr, $bus: expr, $device: expr, $func:expr) => {
        {
            ($base + (((($bus) << 20) | (($device as u32) << 15) | (($func as u32) << 12)) as usize))
        }
    };
}

#[macro_export]
macro_rules! read_register {
    ($addr: expr, $t: ty) => {
        {
            unsafe {($addr as *const $t).read_volatile()}
        }
    };
}

#[macro_export]
macro_rules! write_register {
    ($addr: expr, $t: ty, $value: expr) => {
        {
            unsafe {($addr as *mut $t).write_volatile($value)}
        }
    };
}