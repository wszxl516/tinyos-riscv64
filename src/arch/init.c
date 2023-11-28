#include "init.h"
#include "printf.h"
#include "start.h"

void FUNC_NORETURN cpu_fn() {
  while (1)
    WFI_IDLE();
}

static SECTION(".device") kernel_t _kernel = {
    .cpus = {INIT_CPU(start), INIT_CPU(cpu_fn)}};

void cpu_init(u32 hartid) {
  REG_WRITE_P(mscratch, &_kernel.cpus[hartid]);
  _kernel.cpus[hartid].online = true;
  _kernel.cpus[hartid].bit = machine_bits();
  _kernel.cpus[hartid].extensions = REG_READ_P(misa);
  _kernel.cpus[hartid].vendor_id = vendor_id();
  _kernel.cpus[hartid].machine_id = machine_id();
  _kernel.cpus[hartid].impl_id = machine_impl_id();
  cpustart_t _cpu_fn = _kernel.cpus[hartid].start_fn;
  if (_cpu_fn)
    _cpu_fn();
  return;
}

void dump_cpu_info() {
  for (u32 i = 0; i < SMP_CORE_NUM; i++) {
    pr_debug("Vendor ID: %x.%x.%x\n", _kernel.cpus[i].vendor_id,
             _kernel.cpus[i].machine_id, _kernel.cpus[i].impl_id);
    char extensions[26] = {0};
    machine_extensions_parse(_kernel.cpus[i].extensions, extensions);
    pr_debug("Core: %u, Arch: Riscv%u, Flags: %s \n", i, _kernel.cpus[i].bit,
             extensions);
  }
}