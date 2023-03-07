#include "config.h"
.file "enstry.S"
.balign 4
.option pic
.section .init
 
.option norvc
 
.type start, @function
.global start
start:
	.cfi_startproc
 
.option push
.option norelax
	la gp, global_pointer
.option pop
	/* Setup stack */
	la sp, stack_top
loop:
	/*current core*/
	csrr t0, mhartid
    beq  t0, zero, setup_s_mode
    jal      loop
	
setup_s_mode:
	# set MPP to 1 (supervisor mode)
	li   t0, (0b1 << 11)
    csrw mstatus, t0
	/* Reset satp  disable mmu*/
	csrw satp, zero
	/* Clear the BSS section */
	la t5, bss_start
	la t6, bss_end

bss_clear:
	sd zero, (t5)
	addi t5, t5, 8
	bgeu t5, t6, bss_clear
	la t0, main
	/*exception*/
	csrw mepc, t0
	/* Jump to kernel! */
	tail main
 
	.cfi_endproc
 