ENTRY(_start);
 
base_addr = 0x80000000;
 
SECTIONS {
	. = base_addr;
	.text : ALIGN(4K) {
		PROVIDE(text_start = .);
		*(.init);
		*(.text);
		*(.text.*)
		PROVIDE(text_end = .);
	}
	.root_table : ALIGN(4K) {}
	.bss : ALIGN(4K) {
		PROVIDE(bss_start = .);
		*(.bss);
		*(.bss.*)
		*(.sbss*)
		. += 4096;
		PROVIDE(bss_end = .);
		PROVIDE(global_pointer = .);
	}
	.rodata : ALIGN(4K) {
		PROVIDE(ro_start = .);
        *(.rodata .rodata.*)
		PROVIDE(ro_end = .);
	}
	.data : ALIGN(4K) {
		PROVIDE(data_start = .);
		*(.data);
		*(.sdata);
		*(.data*);
		PROVIDE(data_end = .);
	}
	.stack : ALIGN(4k) {
		PROVIDE(stack_bottom = .);
		. += 4096 * 4;
		PROVIDE(stack_top = .);
	}
	.symbols : ALIGN(4k){
        PROVIDE(symbols_start = .);
        . += 300k;
        PROVIDE(symbols_end = .);
    }
	PROVIDE(test_symbol = .);
	PROVIDE(heap_start = .);
}
