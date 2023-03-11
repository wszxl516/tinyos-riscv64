#ifndef __MM_H__
#define __MM_H__
#include "common.h"
#include "config.h"
typedef void (ld_script_pointer_t)();
extern ld_script_pointer_t stack_top, stack_bottom;
extern ld_script_pointer_t bss_start, bss_end;
extern ld_script_pointer_t heap_start, text_start;
extern ld_script_pointer_t symtab_start;

typedef usize entry_t __USED__;

#define VPN_SIZE  1 << 9

//0b111111111 << 39

#define BASE_MASK 0xffff
#define VIRT_LEN 48
#define VIRT_ADDR_MASK (BASE_MASK << VIRT_LEN)
#define CORRECT_VIRT(addr)  (!(VIRT_ADDR_MASK & addr ^ VIRT_ADDR_MASK))
#define CORRECT_PHY(addr)   (addr < VIRT_ADDR_MASK)
#define CLEAR_BSS() memset(bss_start, (bss_end - bss_start), 0)

#define PRINT_KERNEL_INFO()do \
{\
	pr_info("#########################################################\n"); \
	pr_info("CPU: %x.%x.%x\n", vendor_id(), machine_id(), machine_impl_id()); \
	char extensions[26] = {0}; \
	machine_extensions(extensions); \
	pr_info("CPU Extensions: %s\n", extensions); \
	pr_info("riscv%u, Core: %d\n", machine_bits(), current_core()); \
	pr_info("kernel size: 0x%x\n", heap_start - text_start); \
	pr_info("bss: 0x%x - 0x%x\n",bss_start, bss_end); \
	double kernel_size = (usize)heap_start - (usize)text_start; \
	usize stack_size = (usize)stack_top - (usize)stack_bottom; \
	pr_info("stack 0x%x - 0x%x, size: %u bytes, %f%%\n", stack_bottom, stack_top ,stack_size, (stack_size/kernel_size)*100); \
	pr_info("#########################################################\n"); \
} while (0);

#endif //__MM_H__