use core::arch::asm;
#[inline]
pub fn syscall(id: usize, args: [usize; 6]) -> isize {
    let ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") args[0] => ret,
            in("a1") args[1],
            in("a2") args[2],
            in("a3") args[3],
            in("a4") args[4],
            in("a5") args[5],
            in("a7") id,
        );
    }
    ret
}
#[macro_export]
macro_rules! syscall_args {
    () => {
        [0, 0, 0, 0, 0, 0]
    };
    ($a1:expr) => {
        [$a1, 0, 0, 0, 0, 0]
    };

    ($a1:expr, $a2:expr) => {
        [$a1, $a2, 0, 0, 0, 0]
    };

    ($a1:expr, $a2:expr, $a3:expr) => {
        [$a1, $a2, $a3, 0, 0, 0]
    };

    ($a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        [$a1, $a2, $a3, $a4, 0, 0]
    };

    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        [$a1, $a2, $a3, $a4, $a5, 0]
    };

    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        [$a1, $a2, $a3, $a4, $a5, $a6]
    };
}
