.align 4
.globl m_trap_handler
.extern setup_timer
m_trap_handler:
    .align 4
    csrrw sp, mscratch, sp
    SAVE_G_REG sp
    csrrw sp, mscratch, sp
    call setup_timer
    //trigger a supervisor software interrupt
    li   t0, 1 << 1
    csrw sip, t0
    csrrw sp, mscratch, sp
    csrr t0, mepc
    REST_G_REG sp
    csrrw sp, mscratch, sp
    mret

.align 4
.globl s_trap_handler
.extern handle_trap
s_trap_handler:
    .align 4
    csrw sie, zero
    csrrw t6, sscratch, t6
    SAVE_G_REG t6
    csrr  t5, sscratch
    sd t5, (30)*REG_SIZE(t6)
    csrr t5, sepc
    sd t5, (31)*REG_SIZE(t6)
    mv a0, t6
    csrrw t6, sscratch, t6
    mv a1, ra
    call handle_trap
    csrr t6, sscratch
    ld t5, (31)*REG_SIZE(t6)
    csrw sepc, t5
    li t5, 0xfff
    csrw sie, t5
    REST_G_REG t6
    sret