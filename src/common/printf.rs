#![allow(dead_code)]
use crate::impl_numeric_enum;
impl_numeric_enum! {
    u8,
    #[derive(Eq, PartialEq, Debug)]
    pub Color [
        Red = 91,
        Green = 92,
        Orange = 93,
        Blue = 94,
        Magenta = 95,
        Cyan = 96,
        White = 97,
    ]
}

#[macro_export]
macro_rules! print {
    () => {};
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::device::console::puts(format_args!($fmt $(, $($arg)+)?))
    };
}

#[macro_export]
macro_rules! println {
    () => { print!("\n") };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::device::console::puts(format_args!(concat!($fmt, "\n\r") $(, $($arg)+)?))
    };
}

#[macro_export]
macro_rules! pr_color {
    ($fmt: literal ,$color: expr $(, $($arg: tt)+)?) =>{
        $crate::device::console::puts(
            format_args!(concat!("\x1b[{}m", $fmt, "\x1b[0m"),
            $color
            $(, $($arg)+)?)
        )
    };
}

#[macro_export]
macro_rules! pr_debug {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        #[cfg(feature = "debug")]
        $crate::pr_color!($fmt,
            $crate::common::printf::Color::Blue.into_value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_info {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::common::printf::Color::Green.into_value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_notice {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::common::printf::Color::Blue.into_value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_warn {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::common::printf::Color::Orange.into_value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_err {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
       $crate::pr_color!($fmt,
            $crate::common::printf::Color::Red.into_value()
            $(, $($arg)+)?
            )
    };
}
