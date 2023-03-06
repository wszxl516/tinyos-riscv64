#include "config.h"
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
	csrr   t0, mhartid
    lui    t1, 0
    beq    t0, t1, 2f
1:  wfi
    j      1b
2:
	/* Reset satp */
	/*disable mmu*/
	csrw satp, zero
 
	/* Setup stack */
	la sp, stack_top
 
	/* Clear the BSS section */
	la t5, bss_start
	la t6, bss_end
bss_clear:
	sd zero, (t5)
	addi t5, t5, 8
	bgeu t5, t6, bss_clear
 
	la t0, main
	csrw mepc, t0
 
	/* Jump to kernel! */
	tail main
 
	.cfi_endproc
 
.end
