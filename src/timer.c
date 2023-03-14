#include "timer.h"
#include "riscv.h"
#include "trap.h"
#include "printf.h"

void get_time(time_t *time){

    u64 nsec = REG_READ(RTC_BASE_ADDR + GOLDFISH_RTC_TIME, u64);
	time->sec = nsec / NANO_SECOND;
	time->nsec = nsec % NANO_SECOND;

}


void set_time(time_t *time){

    u64 nsec = (u64)time->sec * NANO_SECOND + time->nsec;
	REG_WRITE(RTC_BASE_ADDR + GOLDFISH_RTC_TIME, u64, nsec >> 32);
	
}


void setup_timer(){
    disable_all_interruput_m();
    time_t time;
    get_time(&time);
    pr_info("timer: %u.%u \n", time.sec, time.nsec);
    REG_WRITE(CLINT_BASE + MTIME_CMP_OFFSET, u64, REG_READ(CLINT_BASE + MTIME_OFFSET, u64) + TIMER_A_SEC);
    enable_all_interruput_m();
}