#ifndef __MM_H__
#define __MM_H__
#include "common.h"
#include "config.h"
typedef void (ld_script_pointer_t)();
extern ld_script_pointer_t stack_top, stack_bottom;
extern ld_script_pointer_t bss_start, bss_end;
extern ld_script_pointer_t heap_start, text_start;


typedef usize entry_t;

#define VPN_SIZE  1 << 9

//0b111111111 << 39

#define BASE_MASK 0xffff
#define VIRT_LEN 48
#define VIRT_ADDR_MASK (BASE_MASK << VIRT_LEN)
#define CORRECT_VIRT(addr)  (!(VIRT_ADDR_MASK & addr ^ VIRT_ADDR_MASK))
#define CORRECT_PHY(addr)   (addr < VIRT_ADDR_MASK)
#endif //__MM_H__