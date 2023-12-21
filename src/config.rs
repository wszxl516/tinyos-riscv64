#![allow(dead_code)]

pub const BASE_UART: usize = 0x10000000;
pub const LOGO_STR: &str = r#"
  _______             __ __                     __
 /_  __(_)___  __  __/ //_/__  _________  ___  / /
  / / / / __ \/ / / / ,< / _ \/ ___/ __ \/ _ \/ /
 / / / / / / / /_/ / /| /  __/ /  / / / /  __/ /
/_/ /_/_/ /_/\__, /_/ |_\___/_/  /_/ /_/\___/_/
            /____/
"#;

pub const SMP_COUNT: usize = 1;

pub const NANO_SECOND: u64 = 1000000000;
//google goldfish
pub const RTC_BASE_ADDR: usize = 0x101000;
pub const GOLDFISH_RTC_TIME: usize = 0x00;
//system realtime clock with 1 second resolution.
pub const A_SECOND: u64 = 1000000;
pub const CLINT_BASE: usize = 0x2000000;
//https://github.com/pulp-platform/clint

pub const PLIC_BASE: usize = 0xc000000;
pub const MTIME_OFFSET: usize = 0xbff8;
pub const MTIME_CMP_OFFSET: usize = 0x4000;
pub const CPU_CLOCK_HZ: u64 = 10000000;
pub const TICK_RATE_HZ: u64 = 1000;
pub const ONE_TICK: u64 = CPU_CLOCK_HZ / TICK_RATE_HZ;
const ONE_MB: usize = 0x100000;
pub const MEM_SIZE: usize = ONE_MB * 64;
