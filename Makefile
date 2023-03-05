
TARGET=kernel.elf
override PWD=$(shell pwd)
override SRC_DIR=$(PWD)/src
override ASM_DIR=$(SRC_DIR)/arch
CC := riscv64-elf-gcc
AS := riscv64-elf-as
EX_CFLAGS := 
OUT_DIR :=$(PWD)/out

override C_SRCS := $(shell find $(SRC_DIR) -name "*.c")
override ASM_SRCS := $(shell find $(ASM_DIR) -name "*.s")
override INCLUDE := $(foreach dir, $(shell find $(PWD)/include -type d), -I$(dir))
override HEADERS := $(shell find $(PWD)/include -name "*.h")

override OBJS = $(foreach file, $(C_SRCS:%.c=%.o) $(ASM_SRCS:%.s=%.o), $(OUT_DIR)/$(shell basename $(file)))
override CFLAGS  = -g -Wall -Wextra -mcmodel=medany -ffreestanding $(INCLUDE) $(EXP_CFLAGS)
override LDFLAGS = -T linker.ld -lgcc -nostdlib -g

define QEMU_ARGS
	-smp 2 \
	-machine virt \
	-bios none \
	-chardev stdio,id=ttys0 \
	-serial chardev:ttys0 \
	-monitor tcp::1122,server,nowait \
	-nographic \
	-kernel $(OUT_DIR)/$(TARGET)
endef
$(OUT_DIR)/%.o: $(SRC_DIR)/%.c $(HEADERS)
	@echo [CC] $<
	@$(CC) -c -o $@ $< $(CFLAGS)

$(OUT_DIR)/%.o: $(ASM_DIR)/%.s $(HEADERS)
	@echo [AS] $<
	@$(AS) -c -o $@ $<

$(TARGET): pre_check $(OBJS) 
	@echo [LINK] $@
	@$(CC) -o $(OUT_DIR)/$(TARGET) $(OBJS) $(CFLAGS)  $(LDFLAGS)

all:pre_check $(TARGET)

run: all
	@qemu-system-riscv64 $(QEMU_ARGS)

pre_check:
	@if [ ! -f "$$(which riscv64-elf-gcc)" ] || \
		[ ! -f "$$(which riscv64-elf-gdb)" ] || \
		[ ! -f "$$(which qemu-system-riscv64)" ];then \
		echo -ne "\033[91mMust install riscv64-elf-gcc, riscv64-elf-gdb and qemu-system-riscv64!\033[0m\n"; \
		exit 1; \
	fi
	@mkdir -p $(OUT_DIR)

debug: all
	@/usr/bin/xfce4-terminal -e \
		'qemu-system-riscv64 $(QEMU_ARGS) -s -S'
	#@lldb -O "target create $(OUT_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"
	@riscv64-linux-gnu-gdb $(OUT_DIR)/$(TARGET) -ex "target remote :1234"

dump_dtb:
	@qemu-system-riscv64 -machine virt -machine dumpdtb=$(OUT_DIR)/riscv64-virt.dtb > /dev/null 2>&1
	@dtc -O dts -o $(OUT_DIR)/riscv64-virt.dts  $(OUT_DIR)/riscv64-virt.dtb > /dev/null 2>&1
	@rm $(OUT_DIR)/riscv64-virt.dtb -f
	@echo "$(OUT_DIR)/riscv64-virt.dts dumped"

clean:
	@rm $(OUT_DIR)/ -rf
