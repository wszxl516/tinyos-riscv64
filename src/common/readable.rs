use alloc::format;
use alloc::string::String;

const SUFFIX: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

pub trait HumanReadable {
    fn readable(&self, keep: usize) -> String;
}

pub fn human_bytes(size: f64, keep: usize) -> String {
    let mut base = size;
    let mut i = 0usize;
    while base > 1024.0f64 {
        base /= 1024.0;
        i += 1;
    }
    format!("{:0.*} {}", keep, base, SUFFIX[i])
}

impl HumanReadable for usize {
    fn readable(&self, keep: usize) -> String {
        human_bytes(*self as f64, keep)
    }
}

impl HumanReadable for u64 {
    fn readable(&self, keep: usize) -> String {
        human_bytes(*self as f64, keep)
    }
}