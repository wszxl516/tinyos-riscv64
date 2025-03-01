# Compiler and Tools
TARGET = tiny-riscv64
PWD=$(shell pwd)
OUT_DIR = $(PWD)/target/riscv64gc-unknown-none-elf/debug
GDB = rust-gdb
QEMU = qemu-system-riscv64
GEN_SYMBOLS = ./parse_symbol.py

# Output suppression
NO_OUTPUT = > /dev/null 2>&1

# QEMU arguments
QEMU_ARGS = \
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

QEMU_ARGS_RUN = $(QEMU_ARGS) -kernel $(OUT_DIR)/$(TARGET).bin

# Rule to generate symbol section
define generate_symbols
    $(GEN_SYMBOLS) $1 $2 $3 -v
endef

# Default target
all: cargo_build

# Build the project
cargo_build:
	cargo build

# Build and generate binary file
$(OUT_DIR)/$(TARGET).bin: all
	@$(call generate_symbols, $(OUT_DIR)/$(TARGET), $(OUT_DIR)/symbol_section , 262144) > symbols.log
	@rust-objcopy --update-section .symbols=$(OUT_DIR)/symbol_section --set-section-flags .symbols=data,contents,alloc,load $(OUT_DIR)/$(TARGET)
	@rust-objcopy --binary-architecture=riscv64 --strip-all -O binary $(OUT_DIR)/$(TARGET) $(OUT_DIR)/$(TARGET).bin

# Run QEMU
run: $(OUT_DIR)/$(TARGET).bin make_fs
	@$(QEMU) $(QEMU_ARGS_RUN)

# Make the file system image
make_fs:
	@dd if=/dev/urandom of=hd.img bs=1M count=64 $(NO_OUTPUT)
	@mkfs.fat -F 32 hd.img $(NO_OUTPUT)
hd.img: make_fs
# Debugging with QEMU and rust-lldb
debug: $(OUT_DIR)/$(TARGET).bin hd.img
	/usr/bin/xfce4-terminal -e '$(QEMU) $(QEMU_ARGS_RUN) -s -S' &
	@rust-lldb -O "target create $(OUT_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"

# Dump device tree blob
dump_dtb: hd.img
	@$(QEMU) $(QEMU_ARGS) -machine dumpdtb=./riscv64-virt.dtb $(NO_OUTPUT)
	@dtc -O dts -o ./riscv64-virt.dts ./riscv64-virt.dtb $(NO_OUTPUT)
	@rm ./riscv64-virt.dtb -f
	@echo "./riscv64-virt.dts dumped"

# Clean the build and temporary files
clean:
	@cargo clean
	@rm -rf hd.img os.map riscv64-virt.dts
