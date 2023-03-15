#ifndef __SPIN_LOCK__
#define __SPIN_LOCK__
#include "stdtypes.h"
#include "timer.h"
typedef struct {
        volatile u32 lock;
} spinlock_t;

#define STATIC_INIT_SPIN_LOCK(name) static spinlock_t name = {.lock = 0}
#define spin_is_locked(x)  (x->lock == 1)
static inline void spin_lock(spinlock_t *lock)
{
    disable_timer();
    while (1) {
        if (spin_is_locked(lock))
                continue;
        lock->lock = 1;
                break;
    }
    enable_timer();
}

static inline void spin_unlock(spinlock_t *lock)
{
	lock->lock = 0;
}

#endif //__SPIN_LOCK__