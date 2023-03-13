#include "config.h"
#define SIZE_16_KB 0x4
.file "enstry.S"
.balign 4
.option pic
.section .init
 
.option norvc
 
.type _start, @function
.global _start
_start:
.cfi_startproc 
.option push
.option norelax
.option pop
	/*current core*/
	csrr a0, mhartid
    beq  a0, zero, first_core
	j other_core

first_core:
	la gp, global_pointer
	/* Setup stack */
	la sp, stack_top
	call   cpu_init
	call   start

other_core:
	/* Set up stack */
	la t0, stack_top
	mv t1, a0
	/* 4096 */
	lui t3, SIZE_16_KB
loop:
	sub t0, t0, t3
	addi t1, t1, - 1
	bnez t1, loop
	mv sp, t0
	j cpu_init
.cfi_endproc