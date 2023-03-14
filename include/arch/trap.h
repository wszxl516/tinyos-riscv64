#ifndef __TRAP_H__
#define __TRAP_H__
#include "common.h"
#include "riscv.h"

extern void trap_handler();
extern void timer_handler();
typedef enum{
    //1 0 RESERVED
    SUPERVISOR_SOFTWARE_INTERRUPT = 1,
    //1 2 RESERVED
    MACHINE_SOFTWARE_INTERRUPT = 3,
    //1 4 RESERVED
    SUPERVISOR_TIMER_INTERRUPT = 5,
    //1 6 RESERVED
    MACHINE_TIMER_INTERRUPT = 7,
    //1 8 RESERVED
    SUPERVISOR_EXTERNAL_INTERRUPT = 9,
    //1 10 RESERVED
    MACHINE_EXTERNAL_INTERRUPT = 11
    //1 12–15 RESERVED
    //1 ≥16 DESIGNATED FOR PLATFORM USE
}interrupt_t;
typedef enum{
    INSTRUCTION_ADDRESS_MISALIGNED = 0,
    INSTRUCTION_ACCESS_FAULT = 1,
    ILLEGAL_INSTRUCTION = 2,
    BREAKPOINT =3,
    LOAD_ADDRESS_MISALIGNED = 4,
    LOAD_ACCESS_FAULT = 5,
    STORE_AMO_ADDRESS_MISALIGNED = 6,
    STORE_AMO_ACCESS_FAULT = 7,
    ENVIRONMENT_CALL_FROM_U_MODE = 8,
    ENVIRONMENT_CALL_FROM_S_MODE = 9,
    //10 RESERVED
    ENVIRONMENT_CALL_FROM_M_MODE = 11,
    INSTRUCTION_PAGE_FAULT = 12,
    LOAD_PAGE_FAULT = 13,
    //14 RESERVED
    STORE_AMO_PAGE_FAULT = 15,
    //16–23 RESERVED
    //24–31 DESIGNATED FOR CUSTOM USE
    //32–47 RESERVED
    //48–63 DESIGNATED FOR CUSTOM USE
    //≥64 RESERVED
} exception_t;

#define SETUP_GENERIC_REG(addr, regs) do{ \
    for (int i = 0; i < 32; i++) \
        regs[i] = *(((usize*)addr) + i); \
} while (0)

void dump_stack(regs_t *regs);
void handle_trap();
#endif //__TRAP_H__