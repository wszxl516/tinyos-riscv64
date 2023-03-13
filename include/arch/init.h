#ifndef __INIT_H__
#define __INIT_H__
#include "common.h"
#include "config.h"
#include "riscv.h"

typedef FUNC_NORETURN void (*cpustart_t)();

typedef struct{
    u32 hartid;
	u32 bit;
	usize extensions;
	usize vendor_id;
	usize machine_id;
	usize impl_id;
	bool online;
    cpustart_t start_fn;
} cpu;

typedef struct{
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
