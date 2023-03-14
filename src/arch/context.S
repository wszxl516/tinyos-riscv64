.include "reg.S"
#define REG_STACK_SIZE (8*32)
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
timer_handler:
        # get 8 * 32 bytes stack to save registers.
        addi sp, sp, -REG_STACK_SIZE
        SAVE_G_REG 1, 31
        # call timer_redirect
        mv s0, sp
        mv a0, sp
        mv a1, ra
        call setup_timer
        # exception pc + 4
        csrr t0, sepc
        addi t0, t0, 4
        csrw sepc, t0
        # restore registers.
        mv sp, s0
        REST_G_REG 1, 31
        addi sp, sp, REG_STACK_SIZE
        # call trap_ret
        mret