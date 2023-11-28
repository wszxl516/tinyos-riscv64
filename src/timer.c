#include "timer.h"
#include "printf.h"
#include "riscv.h"
#include "trap.h"
void get_time(time_t *time) {

  u64 nsec = REG_READ(RTC_BASE_ADDR + GOLDFISH_RTC_TIME, u64);
  time->sec = nsec / NANO_SECOND;
  time->nsec = nsec % NANO_SECOND;
}

void set_time(time_t *time) {
  u64 nsec = (u64)time->sec * NANO_SECOND + time->nsec;
  REG_WRITE(RTC_BASE_ADDR + GOLDFISH_RTC_TIME, u64, nsec >> 32);
}

void setup_timer() {
  REG_WRITE(CLINT_BASE + MTIME_CMP_OFFSET, u64,
            REG_READ(CLINT_BASE + MTIME_OFFSET, u64) + ONE_TICK * 100);
}

void disable_timer() {
  REG_WRITE(CLINT_BASE + MTIME_CMP_OFFSET, u64,
            REG_READ(CLINT_BASE + MTIME_OFFSET, u64) + U64_MAX);
}

void enable_timer() {
  REG_WRITE(CLINT_BASE + MTIME_CMP_OFFSET, u64,
            REG_READ(CLINT_BASE + MTIME_OFFSET, u64) + ONE_TICK * 100);
}