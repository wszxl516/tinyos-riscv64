#[macro_export]
macro_rules! reg_read_raw {
    ($width:ty, $asm_instr:tt, $asm_reg_name:tt) => {
        match (){
        () => {
                let mut value: $width;
                unsafe {
                    core::arch::asm!(
                        concat!(stringify!($asm_instr),
                        " {reg}, ",
                        stringify!($asm_reg_name)),
                        reg = out(reg) value,
                        options(nomem, nostack)
                    )
                };
                value
        }
        }
    }
}

#[macro_export]
macro_rules! reg_write_raw {
    ($asm_instr:tt, $asm_reg_name:tt, $asm_value:expr) => {
        unsafe {
            core::arch::asm!(
                        concat!(stringify!($asm_instr)," ",
                        stringify!($asm_reg_name),
                        ", {reg}"),
                        reg = in(reg) $asm_value,
                        options(nomem, nostack)
            )
        }

    }
}

//privileged register write
#[macro_export]
macro_rules! reg_write_p {
    ($asm_reg_name:tt, $asm_value:expr) => {
        $crate::reg_write_raw!(csrw, $asm_reg_name, ($asm_value))
    };
}

//privileged register read
#[macro_export]
macro_rules! reg_read_p {
    ($asm_reg_name:tt) => {
        $crate::reg_read_raw!(usize, csrr, $asm_reg_name)
    };
}
//privileged register clear
#[macro_export]
macro_rules! reg_clear_bit_p {
    ($asm_reg_name:tt, $asm_value:expr) => {
        $crate::reg_write_raw!(csrc, $asm_reg_name, ($asm_value))
    };
}
//privileged register set bit
#[macro_export]
macro_rules! reg_set_bit_p {
    ($asm_reg_name:tt, $asm_value:expr) => {
        $crate::reg_write_raw!(csrs, $asm_reg_name, ($asm_value))
    };
}
//privileged register update
#[macro_export]
macro_rules! reg_update_p {
    ($asm_reg_name:tt, $asm_value:expr) => {
        let value = $crate::reg_read_raw!(usize, csrr, $asm_reg_name);
        $crate::reg_write_raw!(csrw, $asm_reg_name, ($asm_value | value))
    };
}
//generic register write
#[macro_export]
macro_rules! reg_write_g {
    ($asm_reg_name:tt, $asm_value:expr) => {
        $crate::reg_write_raw!(mv, $asm_reg_name, $asm_value)
    };
}

//generic register read
#[macro_export]
macro_rules! reg_read_g {
    ($asm_reg_name:tt) => {
        $crate::reg_read_raw!(usize, mv, $asm_reg_name)
    };
}

//memory-mapped register read
#[macro_export]
macro_rules! reg_read_a {
    ($reg_addr:expr, $value_type:ty) => {
        unsafe { core::ptr::read_volatile(($reg_addr) as *const $value_type) }
    };
}

//memory-mapped register write
#[macro_export]
macro_rules! reg_write_a {
    ($reg_addr:expr, $value: expr, $value_type:ty) => {
        unsafe { core::ptr::write_volatile(($reg_addr) as *mut $value_type, $value) }
    };
}

#[macro_export]
macro_rules! get_bits {
    ($value: expr, $start:expr, $num_bits:expr) => {
        ($value >> $start) & ((1 << $num_bits) - 1)
    };
}
