#ifndef __HEAP_H__
#define __HEAP_H__
#include "config.h"
#include "mm.h"

#define HEAP_PAGE_NUM (HEAP_SIZE / PAGE_SIZE)
#define HEAP_START ((usize)heap_start)
#define HEAP_END   ((usize)heap_start + HEAP_SIZE)

typedef struct OPTIMIZATION_ALIGN(4){
    u32 start_page;
    u32 n_pages;
} mem_block_t;


typedef struct OPTIMIZATION_ALIGN(4)
{
    mem_block_t used[HEAP_PAGE_NUM];
    u32 used_num;
    bool used_list[HEAP_PAGE_NUM];
    usize start;
    usize end;
} heap_t __USED__;

void heap_init();
void *alloc(u32 size);
void free(void *ptr);
#endif //__HEAP_H__