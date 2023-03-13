#include "printf.h"
#include "mm.h"
#ifdef __RUN_TEST__
	#include "test.h"
#endif //__RUN_TEST__

void main(void) {
	CLEAR_BSS();
	uart_init();
	pr_notice(BOOT_LOGO "\n");
    #ifdef __MEM_INFO__
        PRINT_KERNEL_INFO();
    #endif //__MEM_INFO__

#ifdef __RUN_TEST__
	#include "test.h"
	heap_test();
	exception_test();
	time_test();
	REBOOT();
	SHUTDONW();
#endif //__RUN_TEST__
	while(1) {
		char c = get_c();
		pr_info("key: 0x%x\n", c);
	}
	return;
}