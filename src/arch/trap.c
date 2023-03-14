#include "trap.h"
#include "timer.h"
#include "printf.h"

//riscv-privileged-20211203-2.pdf
void dump_stack(regs_t *regs){
    pr_err("\n");
    pr_err("call stack:\n");
    pr_err("\t#1: %p \n", (void*)regs->sepc);
    pr_err("\t#0: %p \n", (void*)regs->ra);
    pr_err("stval: %x\n", regs->stval);
    pr_err("code: ");
    for (i32 x = -4; x < 4; x++)
    {
        char code = *((char*)regs->sepc+x);
        pr_err("%02x ", code);
    }
    pr_err("\n");
    pr_err("\n");
    u32 i = 0;

    while (i<=30)
    {
        for (u32 x = 0; x < 3 && i<=30; x++)
        {
            pr_err("x%02d = %012p ", i + 1, regs->regs[i]);
            i++;
        }
        pr_err("\n");
    }
    pr_err("\n");
    pr_err("\n");
}

static void interrupt_handler(usize exception_code, regs_t *regs){
        pr_notice("\n", regs->regs[10]);
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
}


static void exception_handler(usize exception_code, regs_t *regs){

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
    dump_stack(regs);
}


void handle_trap() {
    disable_all_interruput_s();
    regs_t regs;
    SETUP_GENERIC_REG(REG_READ_G(a0), regs.regs);
    regs.ra =  REG_READ_G(a1);
    regs.sepc = REG_READ_P(sepc);
    regs.stval = REG_READ_P(stval);

    usize scause = REG_READ_P(scause);
    u32 exception_type = scause >> (64 - 1);
    usize exception_code = scause << 1 >> 1;
    if (exception_type)
    // Interrupt
        interrupt_handler(exception_code, &regs);
    else
    //Exception
        exception_handler(exception_code, &regs);
    pr_err("\n");
    enable_all_interruput_s();
}