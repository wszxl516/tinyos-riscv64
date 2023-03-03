#ifndef __UART_H__
#define __UART_H__
#include "stdtypes.h"
#include "spinlock.h"
#include "common.h"
#include "config.h"
//Data Ready
#define STATUS_DATA_READY    (1 < 0)
//Overrun Error
#define STATUS_OVERRUN_ERROR (1 < 1)
//Parity Error
#define STATUS_PARITY_ERROR  (1 < 2)
//Framing Error
#define STATUS_FRAMING_ERROR (1 < 3)
//Break Interrupt
#define STATUS_BRK_INTERP    (1 < 4)
//Contains the data to be transmitted  Empty
#define STATUS_THR_EMPTY     (1 < 5)
//Transmitter Empty
#define STATUS_TRANS_EMPTY   (1 < 6)
//FIFO data Error
#define STATUS_DATA_ERROR    (1 < 7)
#define REG volatile
//https://opensocdebug.readthedocs.io/en/latest/02_spec/07_modules/dem_uart/uartspec.html

typedef struct NO_OPTIMIZATION_ALIGN{
            REG u8 data;
            REG u8 itr_enable;
            //interp_status when read;
            REG u8 fifo_ctrl;
            REG u8 line_ctrl;
            REG u8 modem_tatus;
            REG u8 line_status;
            
} ns16550a;

static REG ns16550a *UART = (ns16550a*)QEMU_UART_ADDR;
INIT_STATIC_SPIN_LOCK(uart);

#define DATA_READY() (UART->line_status & STATUS_DATA_READY)
#define THR_EMPTY()  (UART->line_status & STATUS_THR_EMPTY)
#define TRANS_EMPTY()  (UART->line_status & STATUS_TRANS_EMPTY)


static inline void uart_init()
{
    spin_lock(&uart_lock);
    //8 bit word length 2^3
    UART->line_ctrl = 0b1;
    //Tx FIFO Reset, Rx FIFO Reset enable FIFO
    UART->fifo_ctrl = 0b1;
    spin_unlock(&uart_lock);
}


static inline char get_c() {
    spin_lock(&uart_lock);
    while(!THR_EMPTY() && 
          !TRANS_EMPTY() && 
          !DATA_READY());
    char c = UART->data;
    spin_unlock(&uart_lock);
    return c;
}

static inline void put_c(char c)
{   
    spin_lock(&uart_lock);
    UART->data = c; 
    spin_unlock(&uart_lock);
} 

#endif//__UART_H__