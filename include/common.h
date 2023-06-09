#ifndef __COMMON_H__
#define __COMMON_H__
#include "stdtypes.h"
#define NO_OPTIMIZATION_ALIGN    __attribute__ ((packed))
#define OPTIMIZATION_ALIGN(n)    __attribute__ ((aligned(n)))
#define SECTION(n)               __attribute__ ((section(n)))
#define __USED__                 __attribute__((used))
#define __UNUSED__               __attribute__((unused))
#define FUNC_NORETURN            __attribute__((__noreturn__))


#define REG volatile
#define REG_WRITE(addr, type, value)      ((*(REG type*)(addr)) = value)
#define REG_READ(addr, type)       (*(REG type*)(addr))
#define REG_WRITE32(addr, value) 	REG_WRITE(addr, u32, value)
#define REG_READ32(addr) 	        REG_READ(addr, u32)
#define GET_BIT(data, bit_sht) ((data >> bit_sht)&1)


typedef struct NO_OPTIMIZATION_ALIGN
{
 usize regs[31];
 usize ra;
 usize sepc;
 usize stval;
}regs_t;


static inline void memset(void *ptr, u32 size, u8 value)
{
    for (u32 i = 0; i < size; i++)
    {
        *(char*)ptr = value;
        ptr ++;
    }
    
}
#endif //__COMMON_H__