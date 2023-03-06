#include "stdtypes.h"
#include "config.h"
#include "uart.h"
#include "printf.h"
#include "cpu.h"
 
void main(void) {
	uart_init();
	pr_notice(BOOT_LOGO);	
	while(1) {
		char c = get_c();
		pr_info("key: 0x%x\n", c);
		pr_info("riscv %u\n", get_machine_bit());
	}
	return;
}