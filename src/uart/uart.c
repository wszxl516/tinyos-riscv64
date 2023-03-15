#include "uart.h"


static SECTION(".device") REG ns16550a *UART = (ns16550a*)QEMU_UART_ADDR;
STATIC_INIT_SPIN_LOCK(SECTION(".device") uart_lock) ;

void uart_init()
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


char get_c() {
    spin_lock(&uart_lock);
    while(!TRANS_EMPTY() || !DATA_READY());
    char c = UART->data;
    spin_unlock(&uart_lock);
    return c;
}

void put_c(char c)
{   
    while (!THR_EMPTY());  
    spin_lock(&uart_lock);
    UART->data = c; 
    spin_unlock(&uart_lock);
} 

void puts(char *buffer)
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