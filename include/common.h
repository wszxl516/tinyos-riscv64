#ifndef __COMMON_H__
#define __COMMON_H__
#define NO_OPTIMIZATION_ALIGN __attribute__ ((packed))
#define OPTIMIZATION_ALIGN    __attribute__ ((aligned (8)))
#endif //__COMMON_H__