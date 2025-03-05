.align 4
.globl m_trap_handler
.extern setup_timer_m
m_trap_handler:
    .align 4
    csrrw sp, mscratch, sp
    addi sp, sp, -256
    SAVE_G_REG sp
    csrr t6, mie
    sd t6, (31*REG_SIZE)(sp)
    csrw mie, zero
    call setup_timer_m
    //trigger a supervisor software interrupt
    li   t0, 1 << 1
    csrw sip, t0
    ld t6, (31*REG_SIZE)(sp)
    csrw mie, t6
    REST_G_REG sp
    addi sp, sp, 256
    csrrw sp, mscratch, sp
    mret

.align 4
.globl s_trap_handler
.extern handle_trap
s_trap_handler:
    .align 4
    csrrw t6, sscratch, t6
    addi t6, t6, -256
    SAVE_G_REG t6
    csrr  t5, sscratch
    sd t5, (30*REG_SIZE)(t6)
    csrr t5, sepc
    sd t5, (31*REG_SIZE)(t6)
    csrr t5, sie
    sd t5, (32*REG_SIZE)(t6)
    csrw sie, zero
    mv a0, t6
    mv a1, ra
    mv sp, t6
    call handle_trap
    mv t6, sp
    ld t5, (31*REG_SIZE)(t6)
    csrw sepc, t5
    ld t5, (32*REG_SIZE)(t6)
    csrw sie, t5
    addi sp, sp, 256
    csrrw sp, sscratch, sp
    REST_G_REG t6
    sret