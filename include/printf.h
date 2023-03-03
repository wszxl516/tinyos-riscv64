#ifndef __PRINTF_H__
#define __PRINTF_H__
#include "stdtypes.h"
#define va_start(v,l)	__builtin_va_start(v,l)
#define va_end(v)	__builtin_va_end(v)
#define va_arg(v,l)	__builtin_va_arg(v,l)
typedef __builtin_va_list va_list;

u32 vsnprintf(char *buf, u32 size, const char *format, va_list vl);
u32 snprintf(char *buf, u32 size, const char *format, ...);
u32 printf(const char *format, ...);
void print_bits(usize size, void *ptr);
int atoi(const char *str);

#define COLOR_RED       "\033[91m"
#define COLOR_GREEN     "\033[92m"
#define COLOR_YELLOW    "\033[93m"
#define COLOR_BLUE      "\033[94m"
#define COLOR_WHITE     "\033[97m"
#define COLOR_END       "\033[0m"


#define pr_info(fmt,arg...) printf(COLOR_WHITE fmt COLOR_END, ##arg)
#define pr_notice(fmt,arg...) printf(COLOR_GREEN fmt COLOR_END, ##arg)
#define pr_debug(fmt,arg...) printf(COLOR_BLUE fmt COLOR_END, ##arg)
#define pr_warn(fmt,arg...) printf(COLOR_YELLOW fmt COLOR_END, ##arg)
#define pr_err(fmt,arg...) printf(COLOR_RED fmt COLOR_END, ##arg)

#endif //__PRINTF_H__