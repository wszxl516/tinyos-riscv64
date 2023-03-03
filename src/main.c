#include "stdtypes.h"
#include "uart.h"
#include "printf.h"
 
void main(void) {
	uart_init();
	pr_notice("Hello world!\n");
	while(1) {
		char c = get_c();
		pr_notice("0x%x\n", c);
	}
	return;
}