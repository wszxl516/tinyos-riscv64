#ifndef __INIT_H__
#define __INIT_H__
#include "common.h"
#include "config.h"
#include "riscv.h"

typedef FUNC_NORETURN void (*arch_cpustart_t)();

typedef struct{
    u32 hartid;
	u32 bit;
	usize extensions;
	bool online;
    arch_cpustart_t start_fn;
} cpu;

typedef struct z_kernel {
	cpu cpus[SMP_CORE_NUM];
} kernel_t;


#define INIT_CPU(_start_fn) \
{ \
	.hartid = 0, \
	.online = false, \
	.start_fn = _start_fn, \
	.bit = 0, \
	.extensions = 0, \
}

void dump_cpu_info();
#endif //__INIT_H__
