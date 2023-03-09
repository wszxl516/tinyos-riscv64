
#include "cpu.h"
#include "printf.h"
//riscv-privileged-20211203-2.pdf
extern void main(void);
extern void trap_handler();
static inline void clear_bss(){
    __asm__("la t5, bss_start\n"
            "la t6, bss_end\n"
            "bss_clear:\n"
            "sd zero, (t5)\n"
            "addi t5, t5, 8\n"
            "bgeu t5, t6, bss_clear\n"
            );
}

void handle_trap() {
    usize mcause = REG_READ_P(mcause), mepc = REG_READ_P(mepc);
    u32 exception_type = mcause >> (64 - 1);
    usize exception_code = mcause & (-1 >> 1);
    // Interrupt
    if (exception_type)
    {
        switch (exception_code)
        {
        case 1:
            pr_err("Supervisor software interrupt.");
            break;
        case 3:
            pr_err("Machine software interrupt.");
            break;
        case 5:
            pr_err("Supervisor timer interrupt.");
            break;
        case 7:
            pr_err("Machine timer interrupt.");
            break;
        case 9:
            pr_err("Supervisor external interru.");
            break;
        case 11:
            pr_err("Machine external interrupt.");
            break;
        default:
            pr_err("Designated for platform us");
            break;
        }
    }else
    {
        switch (exception_code)
        {
        case 0:
            pr_err("Instruction address misaligned.");
            break;
        case 1:
            pr_err("Instruction access fault.");
            break;
        case 2:
            pr_err("Illegal instruction.");
            break;
        case 3:
            pr_err("Breakpoint.");
            break;        
        case 4:
            pr_err("Load address misaligned.");
            break;  
        case 5:
            pr_err("Load access fault.");
            break;  
        case 6:
            pr_err(" Store/AMO address misaligned.");
            break;        
        case 7:
            pr_err("Store/AMO access fault.");
            break;
        case 8:
            pr_err("Environment call from U-mode.");
            break;        
        case 9:
            pr_err("Environment call from S-mode.");
            break;        
        //Reserved     
        case 11:
            pr_err("Environment call from M-mode.");
            break;        
        case 12:
            pr_err("Instruction page fault.");
            break;
        case 13:
            pr_err("Load page fault.");
            break;     
        //14 Reserved  
        case 15:
            pr_err("Store/AMO page fault.");
            break; 
        //6–23 Reserved       
        case 24 ... 31:
            pr_err("Designated for custom use.");
            break;
        //32–47 Reserved      
        case 48 ... 63:
            pr_err("Designated for custom use.");
            break;
        //0 ≥64 Reserveds
        default:
            break;
        }
    }
    pr_err("\n");
    //rest
    REG_WRITE_P(mepc, (mepc + 0x4));
}
void start(){
    /*set thread pointer to 0*/
    REG_WRITE_G(tp, 0);
    /*set MPP to 1 (supervisor mode) */ 
    /*mstatus.MIE = Supervisor*/
    REG_WRITE_P(mstatus, (0b1 << 11));
    /* Reset satp  disable mmu*/
    set_mmu(MMU_BARE, 0, 0);
    // physical memory protection
    REG_WRITE_P(pmpcfg0,0xf);
    REG_WRITE_P(pmpaddr0, 0xffffffffffffffff);
    // delegate all interrupts and exceptions to supervisor mode.
    REG_WRITE_P(medeleg, 0xffff);
    REG_WRITE_P(mideleg, 0xffff);
    // keep each CPU's hartid in its tp register, for cpuid().
    REG_WRITE_G(tp, REG_READ_P(mhartid));
    // /* enable fpu*/
    REG_WRITE_P(sstatus, (0b1 << 13));

    REG_WRITE_P(mtvec, (usize)trap_handler);
    /*machine mode exception*/
    REG_WRITE_P(mepc, (usize)main);
    //clear_bss();
    main();
    asm volatile("mret");
}