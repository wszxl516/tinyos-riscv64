#include "stdtypes.h"
#include "config.h"
#include "uart.h"
#include "printf.h"
#include "cpu.h"
#include "mm.h"
#include "heap.h"

void main(void) {
	uart_init();
	pr_notice(BOOT_LOGO "\n");
	pr_info("#########################################################\n");
	pr_info("CPU: %x.%x.%x\n", vendor_id(), machine_id(), machine_impl_id());
	char extensions[26] = {0};
	machine_extensions(extensions);
	pr_info("CPU Extensions: %s\n", extensions);
	pr_info("riscv%u, Core: %d\n", machine_bits(), current_core());
	pr_info("kernel size: 0x%x\n", heap_start - text_start);
	pr_info("bss: 0x%x - 0x%x\n",bss_start, bss_end);
	double kernel_size = (usize)heap_start - (usize)text_start;
	usize stack_size = (usize)stack_top - (usize)stack_bottom;
	pr_info("stack 0x%x - 0x%x, size: %u bytes, %f%\n", stack_bottom, stack_top ,stack_size, (stack_size/kernel_size)*100);
	pr_info("#########################################################\n");
	char *ptr = (char*)alloc(32);
	ptr[31] = 0;
	for (u32 i = 0; i <  31 ; i++)
		ptr[i] = 'a';
	pr_info("%s\n", ptr);

	char *ptr1 = (char*)alloc(21708);
	ptr1[31] = 0;
	for (u32 i = 0; i <  31 ; i++)
		ptr1[i] = 'a';
	pr_info("%s\n", ptr1);
	free(ptr1);
	free(ptr);
	while(1) {
		char c = get_c();
		pr_info("key: 0x%x\n", c);
	}
	return;
}