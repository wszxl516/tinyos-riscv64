
TARGET=kernel.elf
PWD=$(shell pwd)
OUT_DIR=$(PWD)/out
SRC_DIR=$(PWD)/src
ASM_DIR=$(SRC_DIR)/arch
$(shell mkdir -p $(OUT_DIR))
CC := riscv64-elf-gcc
AS := riscv64-elf-as
C_SRCS := $(shell find $(SRC_DIR) -name "*.c")
ASM_SRCS := $(shell find $(ASM_DIR) -name "*.S")
INCLUDE := $(foreach dir, $(shell find $(PWD)/include -type d), -I$(dir))
HEADERS := $(shell find $(PWD)/include -name "*.h")
OBJS := $(C_SRCS:%.c=%.o) $(ASM_SRCS:%.S=%.o)
ALL_OBJS = $(foreach file, $(OBJS), $(OUT_DIR)/$(shell basename $(file)))
EXP_CFLAGS := 
override CFLAGS = -Wall -Wextra -mcmodel=medany -ffreestanding $(INCLUDE) $(EXP_CFLAGS) -g
override LDFLAGS= -T linker.ld -lgcc -nostdlib -g
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

$(OUT_DIR)/%.o: $(ASM_DIR)/%.S $(HEADERS)
	@echo [AS] $<
	@$(AS) -c -o $@ $<

$(TARGET): $(ALL_OBJS) 
	@echo [LINK] $@
	@$(CC) -o $(OUT_DIR)/$(TARGET) $(ALL_OBJS) $(CFLAGS)  $(LDFLAGS)

all:$(TARGET)
	@echo $(ALL_OBJS)

run: all
	@qemu-system-riscv64 $(QEMU_ARGS)

debug: all
	@/usr/bin/xfce4-terminal -e \
		'qemu-system-riscv64 $(QEMU_ARGS) -s -S'
	@lldb -O "target create $(OUT_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"

dump_dtb:
	@qemu-system-riscv64 -machine virt -machine dumpdtb=$(OUT_DIR)/riscv64-virt.dtb > /dev/null 2>&1
	@dtc -O dts -o $(OUT_DIR)/riscv64-virt.dts  $(OUT_DIR)/riscv64-virt.dtb > /dev/null 2>&1
	@rm $(OUT_DIR)/riscv64-virt.dtb -f
	@echo "$(OUT_DIR)/riscv64-virt.dts dumped"

clean:
	@rm $(OUT_DIR)/ -rf
