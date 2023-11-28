
#include "start.h"
#include "trap.h"
#include "riscv.h"
#include "printf.h"
#include "timer.h"

void switch_to_s_mode(){
    pr_notice("Entering S-mode.\n");
    M_MODE_RET();
}

void FUNC_NORETURN start(){
    //set thread pointer to 0
    REG_WRITE_G(tp, 0);
    // enable fpu
    //set MPP to 1 (supervisor mode)
    //mstatus.MIE = Supervisor
    REG_UPDATE_P(mstatus, (0b1 << 13) | (0b1 << 11));
    REG_UPDATE_P(sstatus, (0b1 << 13) | (0b1 << 11));
    // Reset satp  disable mmu
    set_mmu(MMU_BARE, 0, 0);
    // delegate all interrupts and exceptions to supervisor mode.
    REG_WRITE_P(medeleg, 0xffff);
    REG_WRITE_P(mideleg, 0xffff);
    // keep each CPU's hartid in its tp register, for cpuid().
    REG_WRITE_G(tp, REG_READ_P(mhartid));
    // set trap handler
    REG_WRITE_P(mtvec, (usize)timer_handler);
    REG_WRITE_P(stvec, (usize)trap_handler);
    /*machine mode exception return pointer*/
    REG_WRITE_P(mepc, (usize)main);
    /*supervisor mode exception return pointer*/
    REG_WRITE_P(sepc, (usize)main);
    // main();
    REG_WRITE_P(pmpcfg0, PMP_R | PMP_W | PMP_X | PMP_NAPOT);
    REG_WRITE_P(pmpaddr0, U64_MAX >> 10);
    // setup timer
    // enable all interrupt and exception
    CLEAR_BSS();
    enable_all_interruput_s();
    enable_all_interruput_m();
    setup_timer();
    switch_to_s_mode();
    while (true) WFI_IDLE();
}