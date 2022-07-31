# Building
TARGET := riscv64gc-unknown-none-elf
MODE ?= release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
FS_IMG := ../fat32-fuse/fs.img
SDCARD := /dev/sdb
CPUS ?= 2
	
ifeq ($(MODE), release)
	BUILD_MODE := --release
else
	BUILD_MODE := 
endif


BOARD ?= qemu
# 默认使用opensbi-1.0
SBI ?= opensbi
ifeq ($(SBI), rustsbi)
	BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin
else 
	BOOTLOADER := ../bootloader/fw_jump.bin
endif

fu740_BOOTLOADER_SIZE := 1310720

# KERNEL ENTRY 0x80200000
ifeq ($(BOARD), qemu)
	KERNEL_ENTRY_PA := 0x80200000
else ifeq ($(BOARD), fu740)
	KERNEL_ENTRY_PA := 0x80200000
endif

# Run fu740
fu740-SERIALPORT	= /dev/ttyUSB0
fu740-BURNER	= ../tools/kflash.py

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

build: env $(KERNEL_BIN)

# build: env switch-check $(KERNEL_BIN) asm # fs-img
# build-light: env switch-check $(KERNEL_BIN) 
	
# 反汇编并把文件保存到target/riscv64gc-unknown-none-elf/release/os.S
asm:
	@$(OBJDUMP) -all $(KERNEL_ELF) > kernel.asm

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
# cargo install cargo-binutils --vers =0.3.3
	rustup component add rust-src
	rustup component add llvm-tools-preview

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

user-apps:
	@make -C ../user elf

fs-img: user-apps
	@./makefs.sh

kernel: user-apps
	@echo Platform: $(BOARD)
	@cp src/linker-$(BOARD).ld src/linker.ld
	@cargo build --offline $(BUILD_MODE) --features "board_$(BOARD)"
	@rm src/linker.ld

clean:
	@cargo clean

run: build
ifeq ($(BOARD),qemu)
	qemu-system-riscv64 \
		-machine virt \
		-m 1G	\
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
        -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-smp $(CPUS)
else
	(which $(fu740-BURNER)) || (cd .. && git clone https://github.com/sipeed/kflash.py.git && mv kflash.py tools)
	@cp $(BOOTLOADER) $(BOOTLOADER).copy
	@dd if=$(KERNEL_BIN) of=$(BOOTLOADER).copy bs=$(fu740_BOOTLOADER_SIZE) seek=1
	@mv $(BOOTLOADER).copy $(KERNEL_BIN)
	@sudo chmod 777 $(fu740-SERIALPORT)
	python3 $(fu740-BURNER) -p $(fu740-SERIALPORT) -b 1500000 $(KERNEL_BIN)
	python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct $(fu740-SERIALPORT) 115200
endif

gdb:
	@qemu-system-riscv64 \
		-m 1G	\
		-machine virt -nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-smp $(CPUS) \
		-s -S

monitor:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'
		
# -ex 'add-symbol-file ../user/busybox_lua_testsuites/lmbench_all' \
# -ex 'add-symbol-file ../user/libc-tests/entry-static.exe'\

run-light:
ifeq ($(BOARD),qemu)
	qemu-system-riscv64 \
		-machine virt \
		-m 8G \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
        -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-smp $(CPUS)
else
	(which $(fu740-BURNER)) || (cd .. && git clone https://github.com/sipeed/kflash.py.git && mv kflash.py tools)
	@cp $(BOOTLOADER) $(BOOTLOADER).copy
	@dd if=$(KERNEL_BIN) of=$(BOOTLOADER).copy bs=$(fu740_BOOTLOADER_SIZE) seek=1
	@mv $(BOOTLOADER).copy $(KERNEL_BIN)
	@sudo chmod 777 $(fu740-SERIALPORT)
	python3 $(fu740-BURNER) -p $(fu740-SERIALPORT) -b 1500000 $(KERNEL_BIN)
	python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct $(fu740-SERIALPORT) 115200
endif

HANDIN_BIN := ../os.bin

handin-fu740: build
	@cp $(KERNEL_BIN) $(HANDIN_BIN)

local: user-apps
	@echo Platform: fu740
	@cp src/linker-fu740.ld src/linker.ld
	@cargo build $(BUILD_MODE) --features "local_fu740" --offline
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)
	@rm src/linker.ld
	cp /home/dhz/workspace/oscomp-2022/os/target/riscv64gc-unknown-none-elf/$(MODE)/os.bin ./

move:
	rm -f ~/nfs/os.bin
	sudo cp ./os.bin ~/nfs	

showmount:
	showmount -e 10.251.176.187

nfs-mount:
	sudo mount -t nfs 10.251.176.187:/home/whsu/Documents/nfs ~/nfs

unmount:
	sudo umount ~/nfs

run-vir-fsimg: build
	qemu-system-riscv64 \
			-machine virt \
			-m 1G	\
			-nographic \
			-bios $(BOOTLOADER) \
			-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
			-device loader,file=$(FS_IMG),addr=0x90000000 \
			-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
			-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
			-smp $(CPUS)

# burn-fu740:
# 	@sudo chmod 777 $(fu740-SERIALPORT)
# 	python3 $(fu740-BURNER) -p $(fu740-SERIALPORT) -b 1500000 $(HANDIN_BIN)
# 	python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct $(fu740-SERIALPORT) 115200

.PHONY: build env kernel clean run user-apps fs-img handin
