#include "heap.h"
#define PAGE_TO_ADDR(start_page) (void *)(start_page * PAGE_SIZE + heap.start)
#define ADDR_TO_PAGE(ptr)        (((usize)ptr - HEAP_START) / PAGE_SIZE)
static heap_t heap = {
    .used = {
        {
            .start_page = 0, 
            .n_pages = 0
        }
    },
    .used_num = 0,
    .used_list = {0},
    .start = HEAP_START, 
    .end = HEAP_END
    };

static i32 find_page(i32 n_pages)
{
    i32 i = 0, page_num = 0, end_page = 0;;
    while (i < HEAP_PAGE_NUM)
    {
        if (!heap.used_list[i])
        {
            page_num ++;
            if (page_num == n_pages)
            {
                end_page = i;
                break;
            }
        }
        else
            page_num = 0;
        i++;
    }

    return end_page + 1 - page_num;
}

void *alloc(u32 size){
    i32 pages = size / PAGE_SIZE + (size % PAGE_SIZE ? 1 : 0);
    u32 i = 0;
    i32 start_page = find_page(pages);
    if (start_page < 0)
        return NULL;
    while (i < HEAP_PAGE_NUM)
    {
        if(heap.used[i].n_pages == 0)
        {
            heap.used[i].start_page = start_page;
            heap.used[i].n_pages = pages;
            break;
        }
        i ++;
    }

    for (i32 i = start_page; i < start_page + pages; i++)
        heap.used_list[i] = true;
    heap.used_num += pages;
    return PAGE_TO_ADDR(start_page);
    
}

void free(void *ptr){
    usize page_start = ADDR_TO_PAGE(ptr);
    u32 page_num = 0;
    for (u32 i = 0; i < heap.used_num; i++)
    {
        if (heap.used[i].start_page == page_start)
        {
            page_num = heap.used[i].n_pages;
            heap.used[i].n_pages = 0;
            heap.used[i].start_page = 0;
            break;
        }
    }
    for (u32 i = page_start; i < page_start+ page_num; i++)
        heap.used_list[i] = false;
    heap.used_num -= page_num;
}