#ifndef __COMMON_H__
#define __COMMON_H__
#define NO_OPTIMIZATION_ALIGN    __attribute__ ((packed))
#define OPTIMIZATION_ALIGN(n)    __attribute__ ((aligned(n)))
#define REG volatile
#define GET_BIT(data, bit_sht) ((data >> bit_sht)&1)

#endif //__COMMON_H__