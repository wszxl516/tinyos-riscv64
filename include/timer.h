#ifndef __TIMER_H__
#define __TIMER_H__
#include "common.h"
#include "config.h"
#define NANO_SECOND 1000000000
//google goldfish
#define RTC_BASE_ADDR   0x101000
#define	GOLDFISH_RTC_TIME	0x00
//system realtime clock with 1 second resolution.
#define A_SECOND    1000000

//https://github.com/pulp-platform/clint
#define MTIME_OFFSET 0xbff8
#define MTIME_CMP_OFFSET 0x4000
#define TIMER_A_SEC 0xa00000
typedef struct
{
    u32 sec;
    u64 nsec;
}time_t;

void get_time(time_t *time);
void set_time(time_t *time);
void setup_timer();
#endif //__TIMER_H__