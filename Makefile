
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
OBJS := $(C_SRCS:%.c=%.o) $(ASM_SRCS:%.S=%.o)
ALL_OBJS = $(foreach file, $(OBJS), $(OUT_DIR)/$(shell basename $(file)))
EXP_CFLAGS := 
override CFLAGS = -Wall -Wextra -mcmodel=medany -ffreestanding $(INCLUDE) $(EXP_CFLAGS) 
override LDFLAGS= -T linker.ld -lgcc -nostdlib

$(OUT_DIR)/%.o: $(SRC_DIR)/%.c
	@echo [CC] $<
	$(CC) -c -o $@ $< $(CFLAGS)

$(OUT_DIR)/%.o: $(ASM_DIR)/%.S
	@echo [AS] $<
	$(AS) -c -o $@ $<

$(TARGET): $(ALL_OBJS)
	@echo [LINK] $@
	@$(CC) -o $(OUT_DIR)/$(TARGET) $(ALL_OBJS) $(CFLAGS)  $(LDFLAGS)

all:$(TARGET)
	@echo $(ALL_OBJS)

run: all
	@qemu-system-riscv64 \
		-smp 2 \
		-machine virt \
		-bios none \
		-chardev stdio,id=ttys0 \
		-serial chardev:ttys0 \
		-monitor tcp::1122,server,nowait \
		-nographic \
		-kernel $(OUT_DIR)/$(TARGET)

clean:
	rm $(OUT_DIR)/ -rf
