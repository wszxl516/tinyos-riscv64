use core::arch::asm;

#[allow(named_asm_labels)]
#[naked]
#[no_mangle]
#[link_section = ".init"]
unsafe extern "C" fn _start() -> ! {
    asm!(
    r#"
	.balign 4
	.option pic
	.option norvc
    csrr t0, mhartid
    bne  t0, zero, loop
    la gp, global_pointer
    /* Setup stack */
    la sp, stack_top
    call  {get_args}
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
    get_args = sym super::super::device::get_args,
    options(noreturn),
    )
}
