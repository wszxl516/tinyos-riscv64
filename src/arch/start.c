
#include "start.h"
#include "cpu.h"
#include "printf.h"
//riscv-privileged-20211203-2.pdf


void handle_trap() {
    usize scause = REG_READ_P(scause), sepc = REG_READ_P(sepc);
    u32 exception_type = scause >> (64 - 1);
    usize exception_code = scause & (-1 >> 1);
    // Interrupt
    if (exception_type)
    {
        switch (exception_code)
        {
        case SUPERVISOR_SOFTWARE_INTERRUPT:
            pr_err("Supervisor software interrupt.");
            break;
        case MACHINE_SOFTWARE_INTERRUPT:
            pr_err("Machine software interrupt.");
            break;
        case SUPERVISOR_TIMER_INTERRUPT:
            pr_err("Supervisor timer interrupt.");
            break;
        case MACHINE_TIMER_INTERRUPT:
            pr_err("Machine timer interrupt.");
            break;
        case SUPERVISOR_EXTERNAL_INTERRUPT:
            pr_err("Supervisor external interru.");
            break;
        case MACHINE_EXTERNAL_INTERRUPT:
            pr_err("Machine external interrupt.");
            break;
        default:
            pr_err("Unknown Exception.");
            break;
        }
    }else
    //Exception
    {
        switch (exception_code)
        {
        case INSTRUCTION_ADDRESS_MISALIGNED:
            pr_err("Instruction address misaligned.");
            break;
        case INSTRUCTION_ACCESS_FAULT:
            pr_err("Instruction access fault.");
            break;
        case ILLEGAL_INSTRUCTION:
            pr_err("Illegal instruction.");
            break;
        case BREAKPOINT:
            pr_err("Breakpoint.");
            break;        
        case LOAD_ADDRESS_MISALIGNED:
            pr_err("Load address misaligned.");
            break;  
        case LOAD_ACCESS_FAULT:
            pr_err("Load access fault.");
            break;  
        case STORE_AMO_ADDRESS_MISALIGNED:
            pr_err(" Store/AMO address misaligned.");
            break;        
        case STORE_AMO_ACCESS_FAULT:
            pr_err("Store/AMO access fault.");
            break;
        case ENVIRONMENT_CALL_FROM_U_MODE:
            pr_err("Environment call from U-mode.");
            break;        
        case ENVIRONMENT_CALL_FROM_S_MODE:
            pr_err("Environment call from S-mode.");
            break;        
        //Reserved     
        case ENVIRONMENT_CALL_FROM_M_MODE:
            pr_err("Environment call from M-mode.");
            break;        
        case INSTRUCTION_PAGE_FAULT:
            pr_err("Instruction page fault.");
            break;
        case LOAD_PAGE_FAULT:
            pr_err("Load page fault.");
            break;     
        //14 Reserved  
        case STORE_AMO_PAGE_FAULT:
            pr_err("Store/AMO page fault.");
            break; 
        //6–23 Reserved       
        case 24 ... 31:
        //32–47 Reserved      
        case 48 ... 63:
        //0 ≥64 Reserveds
        default:
            pr_err("Unknown Exception.");
            break;
        }
    }
    pr_err(" address: %p \n", (void*)sepc);
}

void switch_to_s_mode(){
    pr_notice("Entering S-mode.\n");
    M_MODE_RET();
}

void start(){
    //set thread pointer to 0
    REG_WRITE_G(tp, 0);
    // enable fpu
    //set MPP to 1 (supervisor mode)
    //mstatus.MIE = Supervisor
    REG_UPDATE_P(mstatus, (0b1 << 13) | (0b1 << 11));
    REG_UPDATE_P(sstatus, (0b1 << 13) | (0b1 << 11));
    pr_notice(BOOT_LOGO "\n");
    #ifdef __MEM_INFO__
        PRINT_KERNEL_INFO();
    #endif //__MEM_INFO__
    // Reset satp  disable mmu
    set_mmu(MMU_BARE, 0, 0);
    // delegate all interrupts and exceptions to supervisor mode.
    REG_WRITE_P(medeleg, 0xffff);
    REG_WRITE_P(mideleg, 0xffff);
    // keep each CPU's hartid in its tp register, for cpuid().
    REG_WRITE_G(tp, REG_READ_P(mhartid));
    // set trap handler
    REG_WRITE_P(mtvec, (usize)trap_handler);
    REG_WRITE_P(stvec, (usize)trap_handler);
    /*machine mode exception return pointer*/
    REG_WRITE_P(mepc, (usize)main);
    /*supervisor mode exception return pointer*/
    REG_WRITE_P(sepc, (usize)main);
    // main();
    REG_WRITE_P(pmpcfg0, PMP_R | PMP_W | PMP_X | PMP_NAPOT);
    REG_WRITE_P(pmpaddr0, U64_MAX >> 10);
    switch_to_s_mode();
}