
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
/*
0 A Atomic extension
1 B Tentatively reserved for Bit operations extension
2 C Compressed extension
3 D Double-precision floating-point extension
4 E RV32E base ISA
5 F Single-precision floating-point extension
6 G Additional standard extensions present
7 H Reserved
8 I RV32I/64I/128I base ISA
9 J Tentatively reserved for Dynamically Translated Languages extension
10 K Reserved
11 L Tentatively reserved for Decimal Floating-Point extension
12 M Integer Multiply/Divide extension
13 N User-level interrupts supported
14 O Reserved
15 P Tentatively reserved for Packed-SIMD extension
16 Q Quad-precision floating-point extension
17 R Reserved
18 S Supervisor mode implemented
19 T Tentatively reserved for Transactional Memory extension
20 U User mode implemented
21 V Tentatively reserved for Vector extension
22 W Reserved
23 X Non-standard extensions present
24 Y Reserved
25 Z Reserved
*/
static inline void machine_extensions(char *buffer)
{
	usize xlen = 0; 
	__asm__ volatile("csrr %0, misa" :"=r"(xlen));
	for (usize i = 0; i < 26; i++)
	{
		if(i == 7 || i == 10 || i == 14 || i == 17 || i == 22 || i == 24|| i == 25 )
			continue;
		buffer[i] = (xlen & ( 1<<i )) >> i ? 'A' + i: '\1';
	}
}
#endif //__CPU_H__