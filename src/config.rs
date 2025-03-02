#![allow(dead_code)]

pub const BASE_UART: usize = 0x10000000;
pub const PCI_CONFIG_START: usize = 0x30000000;
pub const PCI_CONFIG_SIZE: usize = 0x10000000;
pub const PCI_MEM_START: usize = 0x400000000;
pub const LOGO_STR: &str = r#"
  _______             __ __                     __
 /_  __(_)___  __  __/ //_/__  _________  ___  / /
  / / / / __ \/ / / / ,< / _ \/ ___/ __ \/ _ \/ /
 / / / / / / / /_/ / /| /  __/ /  / / / /  __/ /
/_/ /_/_/ /_/\__, /_/ |_\___/_/  /_/ /_/\___/_/
            /____/
"#;

pub const SMP_COUNT: usize = 1;

//google goldfish
pub const RTC_BASE_ADDR: usize = 0x101000;
//system realtime clock with 1 second resolution.
pub const A_SECOND: u64 = 1000000;
//https://github.com/pulp-platform/clint
pub const CLINT_BASE: usize = 0x2000000;

pub const PLIC_BASE: usize = 0xc000000;
pub const CLOCK_HZ: u64 = 10_000_000;
const ONE_MB: usize = 0x100000;
pub const MEM_SIZE: usize = ONE_MB * 64;

pub mod ld_script_addr {
    #[macro_export]
    macro_rules! lds_address {
        ($name: ident) => {
            ::paste::paste! {
                unsafe { &crate::config::ld_script_addr::[< $name >] as *const usize as usize }
            }
        };
    }
    macro_rules! ld_script_addr {
      ($valtype:ident, $($name: ident), + $(,)?) => {
          extern "C" {
              $(pub static $name: $valtype;)*
          }
      };
  }
    ld_script_addr! {
        usize,
        heap_start,
        base_addr,
        stack_top,
        stack_bottom,
        bss_start,
        bss_end,
        text_start,
        text_end,
        ro_start,
        ro_end,
        data_start,
        data_end,
        symbols_start,
        symbols_end
    }
}
