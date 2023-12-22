///https://wiki.osdev.org/PCI
use super::{
    bar::Bar,
    ids::{dev_type, find},
};
use crate::device::pci::bar::BarType;
use bitflags::bitflags;
use core::fmt::{Debug, Display, Formatter};
use arrayvec::ArrayString;
use fdt::node::FdtNode;
use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum PciMemType {
    Config = 0b00,
    IOSpace = 0b01,
    MemorySpace32 = 0b10,
    MemorySpace64 = 0b11,
}
impl From<u8> for PciMemType {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => PciMemType::Config,
            0b01 => PciMemType::IOSpace,
            0b10 => PciMemType::MemorySpace32,
            0b11 => PciMemType::MemorySpace64,
            _ => unreachable!(),
        }
    }
}
impl From<PciMemType> for u8 {
    fn from(value: PciMemType) -> Self {
        match value {
            PciMemType::Config => 0b00,
            PciMemType::IOSpace => 0b01,
            PciMemType::MemorySpace32 => 0b10,
            PciMemType::MemorySpace64 => 0b11,
        }
    }
}

pub type PciAddr = usize;
pub type PhyAddr = usize;

#[derive(Copy, Clone)]
pub struct PciMem {
    pub pci_addr: PciAddr,
    pub phy_addr: PhyAddr,
    pub size: usize,
    pub pci_type: PciMemType,
    used_size: usize,
}
impl Debug for PciMem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "PciMem: {{ {:#x}, {:#x}, {:#x} {:?} }}",
            self.pci_addr, self.phy_addr, self.size, self.pci_type
        ))
    }
}

impl PciMem {
    pub fn alloc(&mut self, size: usize) -> Option<(PciAddr, PhyAddr)> {
        let ret = (
            PciAddr::from(self.pci_addr + self.used_size),
            PhyAddr::from(self.pci_addr + self.used_size),
        );
        self.used_size += size;
        if self.used_size >= self.size {
            return None;
        }
        Some(ret)
    }
}

#[derive(Copy, Clone)]
pub struct PCIBus {
    pub reg: usize,
    pub reg_size: usize,
    pub mem: [PciMem; 3],
    pub name: ArrayString<64>,
    pub compatible: ArrayString<64>,
    pub device_type: ArrayString<64>,
    pub bus_range: [u32; 2],
}
impl Debug for PCIBus {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("PCIBus {{name: {}, compatible: {},  type: {}, reg: {:#x}, size: {:#x},  mem: {:?},  bus: [{:#x} {:#x}] }}",
            self.name, self.compatible, self.device_type, self.reg, self.reg_size, self.mem, self.bus_range[0], self.bus_range[1]
        ))
    }
}
impl PCIBus {
    pub fn new() -> Self {
        Self {
            reg: 0,
            reg_size: 0,
            mem: [PciMem {
                pci_addr: 0,
                phy_addr: 0,
                size: 0,
                pci_type: PciMemType::Config,
                used_size: 0,
            }; 3],
            name: ArrayString::new_const(),
            compatible: ArrayString::new_const(),
            device_type: ArrayString::new_const(),
            bus_range: [0; 2],
        }
    }
    pub fn from_fdt(node: &FdtNode) -> PCIBus {
        let mut pci = Self::new();
        pci.name.push_str(node.name);
        pci.compatible.push_str(node.property("compatible").unwrap().as_str().unwrap());
        pci.device_type.push_str(node.property("device_type").unwrap().as_str().unwrap());
        let mut ranges = [0u32; 21];
        let p = node.property("reg").unwrap();
        let s = fdt_get_u64!(p.value);
        pci.reg = s.1 as usize;
        let s = fdt_get_u64!(s.0);
        pci.reg_size = s.1 as usize;
        let p = node.property("ranges").unwrap();
        let mut v = p.value;
        for i in 0..ranges.len() {
            let s = fdt_get_u32!(v);
            ranges[i] = s.1;
            v = s.0
        }
        for i in (0..ranges.len()).step_by(7) {
            let pci_mem = PciMem {
                pci_addr: ((ranges[i + 1] as u64) << 32 | ranges[i + 2] as u64) as usize,
                phy_addr: ((ranges[i + 3] as u64) << 32 | ranges[i + 4] as u64) as usize,
                size: ((ranges[i + 5] as u64) << 32 | ranges[i + 6] as u64) as usize,
                pci_type: PciMemType::from((ranges[i] >> 24 & 0b11) as u8),
                used_size: 0,
            };
            pci.mem[i / 7] = pci_mem;
        }
        let p = node.property("bus-range").unwrap();
        let s = fdt_get_u32!(p.value);
        pci.bus_range[0] = s.1;
        let s = fdt_get_u32!(s.0);
        pci.bus_range[1] = s.1;
        pci
    }
    pub fn find_device(&self, vendor_id:  u16, device_id: u16) -> Option<Header0>{
        for device in 0u8..255 {
            match Header0::new(pci_addr!(self.reg, 0, device, 0), 0, device, 0)
             {
                 None => return None,
                 Some(pci) => {
                     if pci.header.vendor_id == vendor_id && pci.header.device_id == device_id {
                         return Some(pci);
                     }
                 }
             }
        }
        return None
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Head {
    pub vendor_id: u16,
    pub device_id: u16,
    command: *mut u16,
    status: *mut u16,
    revision_id: u8,
    prog_if: u8,
    pub sub_class: u8,
    pub class_code: u8,
    cache_line_size: u8,
    latency_timer: u8,
    pub header_type: u8,
    bist: u8,
}
impl Display for Head {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let (v, d) = find(self.vendor_id, self.device_id);
        let s = dev_type(self.class_code, self.sub_class);

        write!(
            f,
            "{} {:#04x}:{:#04x} [ {} {} {:#04x}:{:#04x} ]",
            s, self.class_code, self.sub_class, v, d, self.vendor_id, self.device_id
        )
    }
}

impl Head {
    pub fn read_header(base: usize) -> Head {
        Head {
            vendor_id: read_register!(base + 0x0, u16),
            device_id: read_register!(base + 0x2, u16),
            command: (base + 0x4) as *mut u16,
            status: (base + 0x6) as *mut u16,
            revision_id: read_register!(base + 0x8, u8),
            prog_if: read_register!(base + 0x9, u8),
            sub_class: read_register!(base + 0xa, u8),
            class_code: read_register!(base + 0xb, u8),
            cache_line_size: read_register!(base + 0xc, u8),
            latency_timer: read_register!(base + 0xd, u8),
            header_type: read_register!(base + 0xe, u8),
            bist: read_register!(base + 0xf, u8),
        }
    }
}
impl Display for Header0  {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.header)
    }
}
#[derive(Debug)]
#[repr(C)]
pub struct Header0 {
    pub header: Head,
    pub base_address_reg: [Bar; 6],
    card_bus_cis_pointer: usize,
    subsystem_vendor_id: u16,
    subsystem_id: u16,
    expansion_rom_base_address: *mut u32,
    pub capabilities_pointer: u8,
    interrupt_line: *mut u8,
    interrupt_pin: *mut u8,
    min_grant: u8,
    max_latency: u8,
    pub cap: Option<Cap>,
    pub base_addr: usize,
    pub bus: u8,
    pub device: u8,
    pub func: u8
}
bitflags! {
    struct Command: u16{
        const PCI_COMMAND_IO_EN = 0x0001;
        const PCI_COMMAND_MEM_EN = 0x0002;
        const PCI_COMMAND_BUS_MASTER_EN = 0x0004;
        const PCI_COMMAND_SPECIAL_EN = 0x0008;
        const PCI_COMMAND_MEM_WR_INV_EN = 0x0010;
        const PCI_COMMAND_PAL_SNOOP_EN = 0x0020;
        const PCI_COMMAND_PERR_RESP_EN = 0x0040;
        const PCI_COMMAND_AD_STEP_EN = 0x0080;
        const PCI_COMMAND_SERR_EN = 0x0100;
        const PCI_COMMAND_FAST_B2B_EN = 0x0200;
    }
}
impl Header0 {
    #[allow(dead_code)]
    const SIZE: usize = 0x40;
    pub fn new(base: usize, bus: u8, device: u8, func: u8) -> Option<Self> {
        let header = Head::read_header(base);
        if header.vendor_id == u16::MAX && header.device_id == u16::MAX {
            return None;
        }
        let cap = match read_register!(base + 0x34, u8){
            0 => None,
            _ => Some(Cap::from_addr(base, read_register!(base + 0x34, u8) as usize))
        };
        Some(Self {
            header,
            base_address_reg: [
                Bar::from_addr(base + 0x10),
                Bar::from_addr(base + 0x14),
                Bar::from_addr(base + 0x18),
                Bar::from_addr(base + 0x1c),
                Bar::from_addr(base + 0x20),
                Bar::from_addr(base + 0x24),
            ],
            card_bus_cis_pointer: base + 0x28,
            subsystem_vendor_id: read_register!((base + 0x2a), u16),
            subsystem_id: read_register!((base + 0x2c), u16),
            expansion_rom_base_address: (base + 0x30) as *mut u32,
            capabilities_pointer: read_register!(base + 0x34, u8),
            interrupt_line: (base + 0x3c) as *mut u8,
            interrupt_pin: (base + 0x3d) as *mut u8,
            min_grant: read_register!(base + 0x3e, u8),
            max_latency: read_register!(base + 0x3f, u8),
            base_addr: base,
            bus,
            device,
            cap,
            func,
        })
    }

    pub fn enable(&self) {
        let mut cmd = Command::from_bits(unsafe { self.header.command.read_volatile() }).unwrap();
        cmd |= Command::PCI_COMMAND_IO_EN
            | Command::PCI_COMMAND_MEM_EN
            | Command::PCI_COMMAND_IO_EN
            | Command::PCI_COMMAND_MEM_EN
            | Command::PCI_COMMAND_BUS_MASTER_EN;
        unsafe { self.header.command.write_volatile(cmd.bits()) }
    }
    pub fn setup_bar(&mut self, id: usize, address: usize, cpu_address: usize) {
        // Disable IO and MEM decoding around BAR detection, as we fiddle with
        let cmd = Command::from_bits(unsafe { self.header.command.read_volatile() })
            .unwrap()
            .bits();
        unsafe {
            self.header.command.write_volatile(
                (Command::from_bits(cmd).unwrap()
                    & !(Command::PCI_COMMAND_IO_EN | Command::PCI_COMMAND_MEM_EN))
                    .bits(),
            )
        };
        match self.base_address_reg[id].bar_type {
            BarType::Memory => match self.base_address_reg[id].mem_bits {
                32 => self.base_address_reg[id].setup(address as u32, cpu_address),
                64 => {
                    self.base_address_reg[id].setup(address as u32, cpu_address);
                    self.base_address_reg[id + 1].setup(address.overflowing_shr(32).0 as u32, cpu_address);
                }
                _ => unreachable!(),
            },
            BarType::IO => match self.base_address_reg[id].mem_bits {
                32 => self.base_address_reg[id].setup(address as u32, cpu_address),
                64 => {
                    self.base_address_reg[id].setup(address as u32, cpu_address);
                    self.base_address_reg[id + 1].setup(address.overflowing_shr(32).0 as u32, cpu_address);
                }
                _ => unreachable!(),
            },
        }
        unsafe { self.header.command.write_volatile(cmd) };
    }
}

register_bitfields![u16,
    MC [
        PER_VECTOR_MASKING OFFSET(8) NUMBITS(1),
        IS_64BITS OFFSET(7) NUMBITS(1),
        Multiple_Message_Enable OFFSET(4) NUMBITS(3),
        Multiple_Message_Capable OFFSET(1) NUMBITS(3),
        ENABLE OFFSET(0) NUMBITS(1),
    ]
];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Cap {
    id: u8,
    pub next_pointer: u8,
    message_control: *mut ReadWrite<u16, MC::Register>,
    message_address_low: *mut u32,
    message_address_high: *mut u32,
    message_data: *mut u16,
    mask: u32,
    pending: *mut u32,
    pub base_addr: usize,
}

impl Cap {
    pub fn from_addr(addr: usize, offset: usize) -> Self {
        let address = addr+ offset;
        Self {
            id: read_register!(address, u8),
            next_pointer: read_register!(address + 0x1, u8),
            message_control: (address + 0x2) as *mut ReadWrite<u16, MC::Register>,
            message_address_low: (address + 0x4) as *mut u32,
            message_address_high: (address + 0x8) as *mut u32,
            message_data: (address + 0xc) as *mut u16,
            mask: read_register!(address + 0x10, u32),
            pending: (address + 0x14) as *mut u32,
            base_addr: addr,
        }
    }
}
impl Debug for Cap {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Cap {{ {:#x} {:#x} }}", self.id, self.next_pointer)
    }
}
