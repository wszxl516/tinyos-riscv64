
#ifndef __CPU_H__
#define __CPU_H__
#include "stdtypes.h"
static inline u32 get_machine_bit(){
		usize xlen = 0;
		__asm__ volatile("csrr %0, misa"
						 :"=r"(xlen));
		xlen >>= 62;
		if(xlen == 1)
			return 32;
		else if(xlen == 2)
			return 64;
		else if(xlen == 4)
			return 128;
	return 0;
}

#endif //__CPU_H__