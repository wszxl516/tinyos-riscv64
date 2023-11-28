#include "uart.h"


static SECTION(".device") REG ns16550a *UART = (ns16550a*)QEMU_UART_ADDR;

void uart_init()
{
    UART->itr_enable = 0;
    //8 bit word length 2^3
    UART->line_ctrl = 0xb;
    //Tx FIFO Reset, Rx FIFO Reset enable FIFO
    UART->fifo_ctrl = 0b1;
    //wait uart ready
    while (!(UART->line_status & 0x60));
    
}


char get_c() {
    while(!TRANS_EMPTY() || !DATA_READY());
    char c = UART->data;
    return c;
}

void put_c(char c)
{   
    while (!THR_EMPTY());  
    UART->data = c; 
} 

void puts(char *buffer)
{
    while (!THR_EMPTY());
    while (*buffer)
    {
        UART->data = *buffer; 
        buffer ++;
    }
}