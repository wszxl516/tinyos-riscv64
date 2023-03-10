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
        call handle_trap
        # restore registers.
        mv sp, s0
        REST_G_REG 1, 31
        addi sp, sp, REG_STACK_SIZE
        # csrr ra, sepc
        sret