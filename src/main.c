#include "stdtypes.h"
 
u8 * uart = (u8 *)0x10000000; 
void putchar(i8 c) {
	*uart = c;
	return;
}
 
void print(const i8 * str) {
	while(*str != '\0') {
		putchar(*str);
		str++;
	}
	return;
}
 
void kmain(void) {
	print("Hello world!\r\n");
	while(1) {
		// Read input from the UART
		putchar(*uart);
	}
	return;
}