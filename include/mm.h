#ifndef __MM_H__
#define __MM_H__
typedef void (ld_script_pointer_t)();
extern ld_script_pointer_t stack_top, stack_bottom;
extern ld_script_pointer_t bss_start, bss_end;
extern ld_script_pointer_t mm_set_start, mm_set_end;
extern ld_script_pointer_t heap_start, text_start;


#endif //__MM_H__