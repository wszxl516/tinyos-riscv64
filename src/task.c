#include "task.h"
#include "heap.h"
#include "list.h"
#include "printf.h"
#include "stdtypes.h"
#include "string.h"
static struct list_node task_head;
Task *current = NULL;
void idle() {
  while (1) {
    pr_info("idle\n");
    WFI_IDLE();
  }
}

void task_init() {
  list_initialize(&task_head);
  add_task(idle, "idle", 0);
}

void add_task(void *func, const char *name, u32 id) {
  Task *task = alloc(sizeof(Task));
  usize stack = (usize)alloc(STACK_SIZE);
  memset((void *)stack, 0, STACK_SIZE);
  task->frame[0] = (usize)func;
  task->frame[1] = stack + STACK_SIZE;
  task->id = id;
  task->func = func;
  strcpy(task->name, name);
  task->stack_top = stack;
  list_add_before(&task_head, &task->node);
}
void print_task(Task *task){
  pr_debug("Task: {id: %d name: %s func: %p stack: %p}\n", task->id, task->name, task->func, task->stack_top);
}
void task_switch() {
  if (current == NULL) {
    current = containerof(task_head.next, Task, node);
  } else {
    current = list_next_wrap_type(&task_head, &current->node, Task, node);
  }
  print_task(current);
}
