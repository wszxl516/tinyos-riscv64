ENTRY(_start);
 
. = 0x80000000;
 
SECTIONS {
	/* Include entry point at start of binary */
	PROVIDE(text_start = .);
	.text : ALIGN(4K) {
		*(.init);
		*(.text);
	}
	.bss : ALIGN(4K) {
		PROVIDE(bss_start = .);
		*(.bss);
		. += 4096;
		PROVIDE(global_pointer = .);
		PROVIDE(bss_end = .);
	}
	.rodata : ALIGN(4K) {
		*(.rodata);
	}
	.data : ALIGN(4K) {
		*(.data);
	}
	.device : ALIGN(4K) {
		*(.device);
	}
	.stack : ALIGN(4k) {
		. += 4096;
		PROVIDE(stack_bottom = .);
		/* 2 cores 2 stack */
		. += 4096 * 4 * 2;
		PROVIDE(stack_top = .);
		. += 4096;
	}
	
	PROVIDE(heap_start = .);
}
