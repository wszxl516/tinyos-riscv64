#ifndef __UART_H__
#define __UART_H__
#include "stdtypes.h"
#include "spinlock.h"
#include "common.h"
#include "config.h"
#define STATUS_DATA_READY    0x1

typedef struct NO_OPTIMIZATION_ALIGN{
            u8 data_reg;
            u8 itr_enable_reg;
            u8 fifo_ctrl_reg;
            u16 line_ctrl_reg;
            u8 line_status_reg;
            
} ns16550a;

static ns16550a *UART = (ns16550a*)QEMU_UART_ADDR;
INIT_STATIC_SPIN_LOCK(uart);

#define UART_READY() ((UART->line_status_reg) & STATUS_DATA_READY)

static inline void uart_init()
{
    spin_lock(&uart_lock);
    //8 bit word length
    UART->line_ctrl_reg = 0x3;
    //enable FIFOs
    UART->line_ctrl_reg = 0x1;
    //enable reciever interrupts
    UART->itr_enable_reg = 0x1;
    spin_unlock(&uart_lock);
}


static inline char get_c() {
    spin_lock(&uart_lock);
    while(!UART_READY());
    char c = UART->data_reg;
    spin_unlock(&uart_lock);
    return c;
}

static inline void put_c(char c)
{   
    spin_lock(&uart_lock);
    UART->data_reg = c; 
    spin_unlock(&uart_lock);
} 

#endif//__UART_H__