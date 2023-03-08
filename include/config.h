#ifndef __CONFIG_H__
#define __CONFIG_H__
#define BOOT_LOGO  "TinyOS!"

//ns16550a
#define QEMU_UART_ADDR       0x10000000
#define _1_M                 0x100000
//memory size
#define MEM_SIZE             0x8000000
#define MEM_START            0x8000000
#define HEAP_SIZE            (_1_M * 16)
#define PAGE_SIZE            (1 << 12)
#endif //__CONFIG_H__