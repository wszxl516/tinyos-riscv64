#ifndef __SPIN_LOCK__
#define __SPIN_LOCK__
typedef struct {
        volatile unsigned int lock;
} spinlock_t;

#define READ_ONCE(x)	(*&(x))
#define INIT_STATIC_SPIN_LOCK(name) static spinlock_t name##_lock = {.lock = 0}
#define spin_is_locked(x)  (READ_ONCE((x)->lock) != 0)
static inline void spin_lock(spinlock_t *lock)
{
    while (1) {
        if (spin_is_locked(lock))
                continue;
        lock->lock = 1;
                break;
    }
}

static inline void spin_unlock(spinlock_t *lock)
{
	lock->lock = 0;
}

#endif //__SPIN_LOCK__