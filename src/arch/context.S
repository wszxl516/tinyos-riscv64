.include "reg.S"
#define REG_STACK_SIZE (8*31)
.align 4
.globl trap_handler
trap_handler:
        # get 8 * 32 bytes stack to save registers.
        addi sp, sp, -REG_STACK_SIZE
        SAVE_G_REG 1, 31
        mv s0, sp
        mv a0, sp
        mv a1, ra
        call handle_trap
        # exception pc + 4
        csrr t0, sepc
        addi t0, t0, 4
        csrw sepc, t0
        # restore registers.
        mv sp, s0
        REST_G_REG 1, 31
        addi sp, sp, REG_STACK_SIZE
        sret



.align 4
.globl timer_handler
.extern current
.extern task_switch
.extern setup_timer
timer_handler:
        # get 8 * 32 bytes stack to save registers.
        csrw mie, zero
        csrw sie, zero
        ld t0, current
        beqz t0, no_current
        mv sp, t0
        SAVE_G_REG 1, 31
no_current:
        call setup_timer
        call task_switch
        ld sp, current
        ld t0, 0 ( sp )
        addi t0, t0, 4
        csrw mepc, t0
        REST_G_REG 1, 31    
        li t0,0xfff
        csrw mie, t0
        csrw sie, t0
        mret