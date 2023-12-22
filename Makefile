#!/usr/bin/env bash

TARGET=tiny-riscv64
override PWD = $(shell pwd)
override OUT_DIR :=$(PWD)/target/riscv64gc-unknown-none-elf/debug
override GDB := rust-gdb
override NO_OUTPUT= > /dev/null 2>&1
override QEMU := qemu-system-riscv64
define QEMU_ARGS
	-smp 1 \
	-machine virt \
	-m 64M \
	-cpu rv64 \
	-bios none \
	-chardev stdio,id=ttys0 \
	-serial chardev:ttys0 \
	-monitor tcp::1122,server,nowait \
	-nographic \
	-device virtio-blk-pci,drive=hd0 \
	-drive if=none,file=hd.img,format=raw,id=hd0
endef

define QEMU_ARGS_RUN
	${QEMU_ARGS} \
	-kernel $(OUT_DIR)/$(TARGET).bin
endef

all:
	cargo build

bin: all
	@rust-objcopy --binary-architecture=riscv64 --strip-all -O binary $(OUT_DIR)/$(TARGET) $(OUT_DIR)/$(TARGET).bin

run: bin make_fs
	@qemu-system-riscv64 $(QEMU_ARGS_RUN)

make_fs:
	@dd if=/dev/urandom of=hd.img bs=1M count=64 $(NO_OUTPUT)
	@mkfs.fat -F 32 hd.img > /dev/null $(NO_OUTPUT)
debug: bin make_fs
	/usr/bin/xfce4-terminal -e '$(QEMU) $(QEMU_ARGS_RUN) -s -S'
	#rust-lldb not working
	#@rust-lldb -O "target create $(OUT_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"
	@RUST_GDB=riscv64-linux-gnu-gdb $(GDB) $(OUT_DIR)/$(TARGET) -ex "target remote :1234"

dump_dtb: make_fs
	@$(QEMU) $(QEMU_ARGS) -machine dumpdtb=./riscv64-virt.dtb $(NO_OUTPUT)
	@dtc -O dts -o ./riscv64-virt.dts  ./riscv64-virt.dtb $(NO_OUTPUT)
	@rm ./riscv64-virt.dtb -f
	@echo "./riscv64-virt.dts dumped"

clean:
	@cargo clean
	@rm hd.img os.map riscv64-virt.dts -rf
