#include "mm.h"
#include "printf.h"
#include "task.h"
#ifdef __RUN_TEST__
#include "test.h"
#endif //__RUN_TEST__
void test() {
  while (1) {
    time_t time;
    for (int i = 0; i <= 100; i++) {
      get_time(&time);
      pr_info("test: %d sec: %u, nsec: %u\n", i, time.sec, time.nsec);
      WFI_IDLE();
    }
  }
}
void main(void) {
  uart_init();
  task_init();
  add_task(test, "test", 1);
  pr_notice(BOOT_LOGO "\n");
#ifdef __MEM_INFO__
  PRINT_KERNEL_INFO();
#endif //__MEM_INFO__
  while (1) {
    WFI_IDLE();
  }
  return;
}