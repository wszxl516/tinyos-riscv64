#![allow(dead_code)]

use crate::arch::trap::plic::register_handler;
use arrayvec::ArrayVec;
use core::fmt;
use core::fmt::Write;

use super::uart::ns16550a::Ns16550a;
use super::BASE_UART;

static mut STDIO: Stdio = Stdio {
    0: Ns16550a::from_addr(0),
};

pub fn uart_init() {
    unsafe {
        STDIO = Stdio {
            0: Ns16550a::from_addr(BASE_UART),
        };
        STDIO.0.init();
    }
    register_handler(0x0a, uart_irq_handler);
    unsafe { UART_BUFFER = Some(ArrayVec::new()) }
}

static mut UART_BUFFER: Option<ArrayVec<u8, 256>> = None;

#[no_mangle]
fn uart_irq_handler() {
    unsafe {
        match &mut UART_BUFFER {
            None => {}
            Some(buffer) => {
                if buffer.is_full() {
                    buffer.clear()
                }
                buffer.push(STDIO.0.get_c())
            }
        }
    }
}

struct Stdio(Ns16550a);

impl Stdio {
    pub fn read_char(&self) -> char {
        char::from(self.0.get_c())
    }
    pub fn read_str(&self, buffer: &mut [u8]) {
        for i in 0..buffer.len() {
            buffer[i] = self.read_char() as u8
        }
    }
}

impl Write for Stdio {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c).unwrap()
        }
        Ok(())
    }
    fn write_char(&mut self, c: char) -> fmt::Result {
        unsafe {
            STDIO.0.put_c(c);
        }
        Ok(())
    }
}

pub fn puts(args: fmt::Arguments) {
    unsafe { STDIO.write_fmt(args) }.unwrap()
}

pub fn gets() -> Option<u8> {
    unsafe {
        match &mut UART_BUFFER {
            None => None,
            Some(buffer) => {
                if buffer.is_empty() {
                    None
                } else {
                    buffer.pop()
                }
            }
        }
    }
}
