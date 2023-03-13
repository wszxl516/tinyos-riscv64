#ifndef __UART_H__
#define __UART_H__
#include "stdtypes.h"
#include "spinlock.h"
#include "common.h"
#include "config.h"
//Data Ready
#define STATUS_DATA_READY_SHT    0
//Contains the data transmitted Empty
#define STATUS_THR_EMPTY_SHT     5
//Transmitter Empty
#define STATUS_TRANS_EMPTY_SHT   6
//FIFO data Error
#define STATUS_DATA_ERROR_SHT    7

//https://opensocdebug.readthedocs.io/en/latest/02_spec/07_modules/dem_uart/uartspec.html

typedef struct OPTIMIZATION_ALIGN(1){
            REG u8 data;
            REG u8 itr_enable;
            //interp_status when read;
            REG u8 fifo_ctrl;
            REG u8 line_ctrl;
            REG u8 modem_tatus;
            REG u8 line_status;
            REG u8 modem_status; 
            REG u8 scratch_pad;
} ns16550a;

static SECTION(".device") REG ns16550a *UART = (ns16550a*)QEMU_UART_ADDR;
STATIC_INIT_SPIN_LOCK(SECTION(".device") uart_lock) ;

#define DATA_READY() (GET_BIT(UART->line_status, STATUS_DATA_READY_SHT) == 1)
#define THR_EMPTY()  (GET_BIT(UART->line_status, STATUS_THR_EMPTY_SHT) == 1)
#define TRANS_EMPTY()  (GET_BIT(UART->line_status, STATUS_TRANS_EMPTY_SHT) == 1)


static inline void uart_init()
{
    spin_lock(&uart_lock);

    UART->itr_enable = 0;
    //8 bit word length 2^3
    UART->line_ctrl = 0xb;
    //Tx FIFO Reset, Rx FIFO Reset enable FIFO
    UART->fifo_ctrl = 0b1;
    //wait uart ready
    while (!(UART->line_status & 0x60));
    
    spin_unlock(&uart_lock);
}


static inline char get_c() {
    spin_lock(&uart_lock);
    while(!TRANS_EMPTY() || !DATA_READY());
    char c = UART->data;
    spin_unlock(&uart_lock);
    return c;
}

static inline void put_c(char c)
{   
    while (!THR_EMPTY());  
    spin_lock(&uart_lock);
    UART->data = c; 
    spin_unlock(&uart_lock);
} 

static inline void puts(char *buffer)
{
    while (!THR_EMPTY());
    spin_lock(&uart_lock);
    while (*buffer)
    {
        UART->data = *buffer; 
        buffer ++;
    }
    spin_unlock(&uart_lock);
}
#endif//__UART_H__