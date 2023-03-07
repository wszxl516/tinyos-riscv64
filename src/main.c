#include "stdtypes.h"
#include "config.h"
#include "uart.h"
#include "printf.h"
#include "cpu.h"
#include "mm.h"

void main(void) {
	uart_init();
	pr_notice(BOOT_LOGO "\n");
	pr_info("################\n");
	pr_info("CPU: %x.%x.%x\n", vendor_id(), machine_id(), machine_impl_id());
	char extensions[26] = {0};
	machine_extensions(extensions);
	pr_info("CPU Extensions: %s\n", extensions);
	pr_info("riscv%u, Core: %d\n", machine_bits(), current_core());
	pr_info("stack: 0x%x - 0x%x\n",stack_top, stack_bottom);
	pr_info("bss: 0x%x - 0x%x\n",bss_start, bss_end);
	pr_info("mm_set: 0x%x - 0x%x\n",mm_set_start, mm_set_end);
	pr_info("heap start: 0x%x\n", heap_start);
	while(1) {
		char c = get_c();
		pr_info("key: 0x%x\n", c);
	}
	return;
}