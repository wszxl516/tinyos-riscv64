#[macro_use]
pub mod macros;
pub mod pci;
pub mod ids;
mod bar;

use super::DTB_ADDR;
use super::virtio::{VirtioBlkTrans, blk};
use fdt;
use pci::PCIBus;
use crate::{pr_err, pr_notice};
use super::super::common::readable::HumanReadable;
static mut PCI_BUS: Option<PCIBus> = None;

pub fn init_pci() {
    let dtb = unsafe { fdt::Fdt::from_ptr(DTB_ADDR as *const u8) }.unwrap();
    if let Some(soc) = dtb.find_node("/soc") {
        for c in soc.children() {
            if c.name.contains("pci@") {
                unsafe { PCI_BUS = Some(PCIBus::from_fdt(&c)) };
            }
        }
    }
}
const MBR_MAGIC: [u8; 2] = [0x55, 0xAA];
pub fn find_virt() {
    let pci_bus;
    match unsafe {PCI_BUS} {
        None => panic!("no pci bus!"),
        Some(p) => {
            pci_bus = p
        }
    }
    match &mut VirtioBlkTrans::from_pci(pci_bus) {
        None => {}
        Some(virt) =>  {
            match &mut blk::VirtIOBlk::new(virt){
                Err(e) => {
                    pr_err!("init block device failed: {:?}\n", e)
                }
                Ok(blk) => {
                    let mut buffer = [0u8; 512];
                    blk.read_block(0, &mut buffer).unwrap();
                    pr_notice!("disk size: {}!\n", (blk.capacity * blk.blk_size).readable(2));
                    if buffer.ends_with(&MBR_MAGIC) {
                        pr_notice!("This is a MBR partition!\n");
                    }
                }
            }

        }
    }

}