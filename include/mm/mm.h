#ifndef __MM_H__
#define __MM_H__
#include "common.h"
#include "config.h"
#include "init.h"
#include "string.h"
typedef void (ld_script_pointer_t)();
extern ld_script_pointer_t stack_top, stack_bottom;
extern ld_script_pointer_t bss_start, bss_end;
extern ld_script_pointer_t heap_start, text_start;
extern ld_script_pointer_t symtab_start;

#define CLEAR_BSS() memset(bss_start, 0, (bss_end - bss_start))

#define PRINT_KERNEL_INFO()do \
{\
	pr_debug("#########################################################\n"); \
	dump_cpu_info();	\
	pr_debug("kernel size: 0x%x\n", heap_start - text_start); \
	pr_debug("bss: 0x%x - 0x%x\n",bss_start, bss_end); \
	pr_debug("stack 0x%x - 0x%x, size: 0x%x\n", stack_bottom, stack_top ,(usize)stack_top - (usize)stack_bottom); \
	pr_debug("heap  0x%x - 0x%x, size: 0x%x\n", heap_start, heap_start + HEAP_SIZE ,HEAP_SIZE); \
	pr_debug("#########################################################\n"); \
} while (0);

#endif //__MM_H__