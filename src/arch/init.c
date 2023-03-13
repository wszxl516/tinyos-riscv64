#include "init.h"
#include "printf.h"
#include "start.h"

void FUNC_NORETURN cpu_fn(){
		while (1) WFI_IDLE();
}

static kernel_t _kernel = {
	.cpus = {
		INIT_CPU(start),
		INIT_CPU(cpu_fn)
	}
};

void cpu_init(u32 hartid)
{
	u32 i;
	u32 cpu_num = 0;
	for (i = 0; i < SMP_CORE_NUM; i++) {
		if (_kernel.cpus[i].hartid == hartid) cpu_num = i;
	}
	REG_WRITE_P(mscratch, &_kernel.cpus[cpu_num]);
	_kernel.cpus[cpu_num].online = true;
	_kernel.cpus[cpu_num].bit = machine_bits();
	_kernel.cpus[cpu_num].extensions = REG_READ_P(misa);
	arch_cpustart_t _cpu_fn = _kernel.cpus[cpu_num].start_fn;
	if (_cpu_fn)
    	_cpu_fn();
	return;
}

void dump_cpu_info(){
	for (u32 i = 0; i < SMP_CORE_NUM; i++)
	{
		char extensions[26] = {0};
		machine_extensions_parse(_kernel.cpus[i].extensions, extensions);
		pr_info("Core: %u, Architecture: Riscv%u, Flags: %s \n", i,  _kernel.cpus[i].bit, extensions);
	}
}