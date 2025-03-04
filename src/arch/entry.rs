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
    csrw mie, zero
    csrw sie, zero
    
    la t0, bss_start
    la t1, bss_end
bss_clear:
    sd  x0, 0(t0)
    add t0, t0, 8
    blt t0, t1, bss_clear

    la t0, {args}
    sd a0, 0(t0)
    sd a1, 8(t0)
    sd a2, 16(t0)

    csrr t0, mhartid
    bne  t0, zero, loop
    la   gp, global_pointer
    /* Setup stack */
    la sp, stack_top
    call  {init}
    call  {setup_trap}

loop:
    wfi
    j   loop
	"#,
    init = sym crate::arch::cpu::init,
    setup_trap = sym crate::arch::trap::setup_trap,
    args = sym super::BOOT_ARGS,
    )
}
