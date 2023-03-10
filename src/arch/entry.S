#include "config.h"
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
loop:
	/*current core*/
	csrr t0, mhartid
    beq  t0, zero, start_label
    jal     loop 
start_label:
	la gp, global_pointer
	/* Setup stack */
	la sp, stack_top
	call   start
.cfi_endproc