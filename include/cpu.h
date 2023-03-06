
#ifndef __CPU_H__
#define __CPU_H__
#include "stdtypes.h"
//riscv-privileged-20211203.pdf

static inline u32 machine_bits()
{
	usize xlen = 0; 
	__asm__ volatile("csrr %0, misa" :"=r"(xlen));
	xlen >>= 62;
	return xlen == 1 ? 32 : xlen == 2 ? 64 : 128;
}

static inline usize current_core()
{
	usize xlen = 0; 
	__asm__ volatile("csrr %0, mhartid" :"=r"(xlen)); 
	return xlen;
}

static inline u32 vendor_id()
{
	u32 id = 0; 
	__asm__ volatile("csrr %0, mvendorid" :"=r"(id)); 
	return id;
}

static inline usize machine_id()
{
	usize id = 0; 
	__asm__ volatile("csrr %0, marchid" :"=r"(id)); 
	return id;
}

static inline usize machine_impl_id()
{
	usize id = 0; 
	__asm__ volatile("csrr %0, mimpid" :"=r"(id)); 
	return id;
}

#endif //__CPU_H__