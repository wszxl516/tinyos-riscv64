#include "stdtypes.h"
#include "config.h"
#include "uart.h"
#include "printf.h"
 
void main(void) {
	uart_init();
	pr_notice(BOOT_LOGO "\n");	
	while(1) {
		char c = get_c();
		pr_info("key: 0x%x\n", c);
	}
	return;
}