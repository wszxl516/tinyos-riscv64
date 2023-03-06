#include "stdtypes.h"
#include "config.h"
#include "uart.h"
#include "printf.h"
#include "cpu.h"
 
void main(void) {
	uart_init();
	pr_notice(BOOT_LOGO "\n");
	pr_info("################\n");
	pr_info("CPU: %x.%x.%x\n", vendor_id(), machine_id(), machine_impl_id());
	pr_info("riscv%u, Core: %d\n", machine_bits(), current_core());
	while(1) {
		char c = get_c();
		pr_info("key: 0x%x\n", c);
	}
	return;
}