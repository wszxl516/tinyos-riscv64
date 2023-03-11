#include "stdtypes.h"
#include "config.h"
#include "uart.h"
#include "printf.h"
#include "board.h"
#include "mm.h"
#include "riscv.h"
#ifdef __RUN_TEST__
	#include "test.h"
#endif //__RUN_TEST__

void main(void) {
	CLEAR_BSS();
	uart_init();
#ifdef __RUN_TEST__
	#include "test.h"
	heap_test();
	exception_test();
	REBOOT();
	SHUTDONW();
#endif //__RUN_TEST__
	while(1) {
		char c = get_c();
		pr_info("key: 0x%x\n", c);
	}
	return;
}