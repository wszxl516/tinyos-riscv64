use core::arch::naked_asm;

#[allow(named_asm_labels)]
#[naked]
#[no_mangle]
#[link_section = ".init"]
unsafe extern "C" fn _start() -> ! {
    naked_asm!(
    r#"
	.balign 4
	.option pic
	.option norvc
    la t0, {args}
    sd a0, 0(t0)
    sd a1, 8(t0)
    sd a2, 16(t0)
    csrr t0, mhartid
    bne  t0, zero, loop
    la gp, global_pointer
    /* Setup stack */
    la sp, stack_top
    call  {init}
    call  {start}
loop:
    sub t0, t0, t3
    addi t1, t1, - 1
    bnez t1, loop
    mv sp, t0
    call {init}
	"#,
    init = sym crate::arch::cpu::init,
    start = sym crate::arch::trap::setup_trap,
    args = sym super::BOOT_ARGS,
    )
}
