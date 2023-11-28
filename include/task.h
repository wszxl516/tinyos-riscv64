#ifndef __TASK_H__
#define __TASK_H__
#include "common.h"
#include "printf.h"
#include "heap.h"
#include "list.h"

#define STACK_SIZE 1024

typedef struct{
    usize frame[31];
    char name[16];
    u32 id;
    void (*func)(void);
    usize stack_top;
    struct list_node node;
} Task;
void task_init();
void add_task(void *func, const char *name,  u32 id);
#endif