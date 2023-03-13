#ifndef __TEST_H__
#define __TEST_H__
#include "heap.h"
#include "printf.h"
#include "board.h"
#include "timer.h"

#define HEAP_TEST(size)  \
do{ \
    char *ptr = (char*)alloc(size); \
    if (!ptr) \
        pr_notice("Test0 alloc 0x%x byte(s) ptr: %p\n", size, (usize)ptr); \
    else{ \
        memset(ptr, size, 0); \
        pr_notice("Test0 alloc 0x%x byte(s) ptr: %p\n", size, (usize)ptr); \
        } \
    char *ptr1 = (char*)alloc(size); \
    if (!ptr1) \
        pr_info("Test1 alloc 0x%x byte(s) ptr: %p\n", size, (usize)ptr1); \
    else{ \
        memset(ptr, size, 0); \
        pr_info("Test1 alloc 0x%x byte(s) ptr: %p\n", size, (usize)ptr1); \
        free(ptr1); \
        } \
    free(ptr); \
}while(0)

static inline void heap_test()
{
    for (usize i = 0; i < 20; i++)
        HEAP_TEST(0x100000 * i);
}


void exception_test()
{
    pr_notice("Test Load access fault.\n");
    char a __USED__= *((char*)(1<<64));
    pr_notice("Test Environment call.\n");
	__asm__ volatile("ecall");
}


void time_test(){
    time_t time;
	get_time(&time);
	pr_info("sec: %u, nsec: %u\n", time.sec, time.nsec);
}
#endif //__TEST_H__