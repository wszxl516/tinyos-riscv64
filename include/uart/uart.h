#ifndef __UART_H__
#define __UART_H__
#include "stdtypes.h"
#include "spinlock.h"
#include "common.h"
#include "config.h"
#include "riscv.h"
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

#define DATA_READY() (GET_BIT(UART->line_status, STATUS_DATA_READY_SHT) == 1)
#define THR_EMPTY()  (GET_BIT(UART->line_status, STATUS_THR_EMPTY_SHT) == 1)
#define TRANS_EMPTY()  (GET_BIT(UART->line_status, STATUS_TRANS_EMPTY_SHT) == 1)

void uart_init();
char get_c();
void put_c(char c);
void puts(char *buffer);

#endif//__UART_H__