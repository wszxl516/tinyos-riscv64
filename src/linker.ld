ENTRY(_start);
 
base_addr = 0x80000000;
 
SECTIONS {
	. = base_addr;
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
	}
	.rodata : ALIGN(4K) {
		PROVIDE(ro_start = .);
        *(.rodata .rodata.*)
	}
	.data : ALIGN(4K) {
		PROVIDE(data_start = .);
		*(.data);
		*(.sdata);
		*(.data*);
	}
	.stack : ALIGN(4k) {
		. += 4096;
		PROVIDE(stack_bottom = .);
		. += 4096 * 2;
		PROVIDE(stack_top = .);
		. += 4096;
	}
	.frame : ALIGN(4k) {
	    PROVIDE(frame_start = .);
	}
	PROVIDE(heap_start = .);
}
