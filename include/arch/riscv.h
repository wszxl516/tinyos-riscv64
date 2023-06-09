
#ifndef __CPU_H__
#define __CPU_H__
#include "stdtypes.h"
//riscv-privileged-20211203.pdf
//read generic register
#define REG_READ_G(name)  ({ \
	usize value = 0; \
	__asm__ volatile("mv %0," #name :"=r"(value)); \
	value; \
	})

//write generic register
#define REG_WRITE_G(name, value) ({ \
	__asm__ volatile("mv " #name ", %0" : : "r"(value)); \
})

//read Privileged register
#define REG_READ_P(name)  ({ \
	usize value = 0; \
	__asm__ volatile("csrr %0," #name :"=r"(value)); \
	value; \
	})

//write Privileged register
#define REG_WRITE_P(name, value) ({ \
	__asm__ volatile("csrw " #name ", %0" : : "r"(value)); \
})

#define REG_UPDATE_P(name, value) REG_WRITE_P(name, (REG_READ_P(name) | value))
#define REG_UPDATE_G(name, value) REG_WRITE_G(name, (REG_READ_G(name) | value))

#define M_MODE_RET()  __asm__ volatile("mret")
#define S_MODE_RET()  __asm__ volatile("sret")
#define SLEEP_CPU()   __asm__ volatile("wfi")

static inline u32 machine_bits()
{
	usize xlen = 0; 
	xlen = REG_READ_P(misa);
	xlen >>= 62;
	return xlen == 1 ? 32 : xlen == 2 ? 64 : 128;
}

static inline usize current_core()
{
	return REG_READ_P(mhartid);
}

static inline u32 vendor_id()
{
	return REG_READ_P(mvendorid);
}

static inline usize machine_id()
{
	return REG_READ_P(marchid);
}

static inline usize machine_impl_id()
{
	return REG_READ_P(mimpid);
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
static inline void machine_extensions_parse(usize len, char *buffer)
{
	for (usize i = 0; i < 26; i++)
	{
		if(i == 7 || i == 10 || i == 14 || i == 17 || i == 22 || i == 24|| i == 25 )
			continue;
		buffer[i] = (len & ( 1<<i )) >> i ? 'A' + i: '\1';
	}
}

//Supervisor Address Translation and Protection

#define MMU_BARE 0b0000L << 60
#define MMU_Sv39 0b1000L << 60
#define MMU_Sv48 0b1001L << 60
#define MMU_Sv57 0b1010L << 60
#define MMU_Sv64 0b1011L << 60
#define MMU_ASID 0      << 44
static inline void set_mmu(usize mode, usize asid , usize address){
	REG_WRITE_P(satp, mode | asid | address);
}


static inline void enable_all_interruput_m()
{
    REG_WRITE_P(mie,  0xfff);
}

static inline void disable_all_interruput_m()
{
    REG_WRITE_P(mie,  0);
}


static inline void enable_all_interruput_s()
{
    REG_WRITE_P(sie,  0xfff);
}


static inline void disable_all_interruput_s()
{
    REG_WRITE_P(sie,  0);
}

#define WFI_IDLE() __asm__ volatile("wfi")
#endif //__CPU_H__