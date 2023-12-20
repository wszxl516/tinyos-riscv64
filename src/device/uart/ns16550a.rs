#![allow(dead_code)]

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::ReadWrite;
use tock_registers::{register_bitfields, register_structs};

//https://opensocdebug.readthedocs.io/en/latest/02_spec/07_modules/dem_uart/uartspec.html
register_structs! {
    ns16550a_reg {
        //000 Character RW
        (0x0 => pub data: ReadWrite<u8>),
        //001 Interrupt Enable Register R/W
        (0x1 => pub interrupt_enable: ReadWrite<u8>),
        //010 FIFO Control Register W
        (0x2 => pub fifo_control: ReadWrite<u8>),
        //011 Line Control Register R/W
        (0x3 => pub line_control: ReadWrite<u8>),
        //100 Modem Control Register R/W
        (0x4 => pub modem_control: ReadWrite<u8>),
        //101 Line Status Register R
        (0x5 => pub line_status: ReadWrite<u8, LINE_STATUS::Register>),
        //110 Modem Status Register R
        (0x6 => pub modem_status: ReadWrite<u8>),
        //111 Scratch Pad Register R/W
        (0x7 => pub scratch_pad: ReadWrite<u8>),
        (0x8 => @END),
    }
}

register_bitfields![u8,
        pub LINE_STATUS [
            READ_READY  OFFSET(0) NUMBITS(1) [
                READY = 1,
                NOT_READY = 0
            ],
            THR OFFSET(5) NUMBITS(1) [
                EMPTY = 1,
                FULL = 0
            ],
            TRANS OFFSET(6) NUMBITS(1) [
                EMPTY = 1,
                FULL = 0
            ]
    ]
];
pub struct Ns16550a(usize);

impl Ns16550a {
    pub(crate) const fn from_addr(addr: usize) -> Self {
        Self { 0: addr }
    }
    #[inline]
    fn reg(&self) -> &ns16550a_reg {
        unsafe { &*(self.0 as *mut ns16550a_reg) }
    }
    pub fn init(&self) {
        self.reg().interrupt_enable.set(1);
        self.reg().line_control.set(0xb);
        self.reg().fifo_control.set(1);
        //wait uart ready
        while !self
            .reg()
            .line_status
            .matches_all(LINE_STATUS::THR::EMPTY + LINE_STATUS::TRANS::EMPTY)
        {}
    }

    pub fn get_c(&self) -> u8 {
        while !self.trans_empty() || !self.data_ready() {}
        self.reg().data.get()
    }

    pub fn put_c(&self, c: char) {
        while !self.thr_empty() {}
        self.reg().data.set(c as u8);
    }

    #[inline]
    fn data_ready(&self) -> bool {
        self.reg()
            .line_status
            .matches_all(LINE_STATUS::READ_READY::READY)
    }
    #[inline]
    fn thr_empty(&self) -> bool {
        self.reg().line_status.matches_all(LINE_STATUS::THR::EMPTY)
    }
    #[inline]
    fn trans_empty(&self) -> bool {
        self.reg()
            .line_status
            .matches_all(LINE_STATUS::TRANS::EMPTY)
    }
}
