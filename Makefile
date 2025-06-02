export cwd := $(shell pwd)

#########
# edited by user.

export MODE ?= debug

#######
### edit PAYLOAD - Select a demo binary file in $(cwd)/payloads directory.
#######
export PAYLOAD ?= $(cwd)/payloads/logging_test_qemu_aarch64.bin

#######
### edit BOARD - Select a platform. Platform list can be found in $(cwd)/codes/arch.
#######
# export BOARD ?= qemu_vexpressa9_armv7
# export BOARD ?= qemu_virt_riscv64
# export BOARD ?= allwinner_D1H
export BOARD ?= qemu_virt_aarch64
# export BOARD ?= raspberry_pi4

#######
### edit TARGET - Your installed cross-platform toolchain and binutils. 
#######
# export TARGET = armv7a-none-eabi
export TARGET = aarch64-unknown-none
# export TARGET = riscv64gc-unknown-none-elf

# export OBJDUMP = rust-objdump --arch-name=riscv64
# export OBJCOPY = rust-objcopy --binary-architecture=riscv64
export OBJDUMP = rust-objdump --arch-name=aarch64
export OBJCOPY = rust-objcopy --binary-architecture=aarch64
# export OBJDUMP = rust-objdump --arch-name=armv7a
# export OBJCOPY = rust-objcopy --binary-architecture=armv7a

##########
# path

export ARCH_PATH := $(cwd)/codes/arch/$(BOARD)
export BUILD_PATH := $(cwd)/build/$(BOARD)
export OUTPUT_PATH := $(cwd)/build/$(BOARD)
export SRC_PATH := $(cwd)/codes/ekc
export MMK_BIN := $(OUTPUT_PATH)/MMK_$(BOARD).bin
export PAYLOAD_BIN := $(PAYLOAD)

#########

	
all: run

run: $(MMK_BIN) $(PAYLOAD_BIN)
	cd $(ARCH_PATH) && sudo sh run.sh $(MMK_BIN) $(PAYLOAD_BIN)

$(MMK_BIN):
	mkdir -p $(BUILD_PATH) 
	cd $(SRC_PATH) && make build

env:
	rustup update
	cargo install cargo-binutils
	cd $(SRC_PATH) && make env

rudra:
	cd $(SRC_PATH) && make rudra

test: $(MMK_BIN)
	cd $(SRC_PATH) && make tmp_run

get_dts:
	qemu-system-arm \
		-M vexpress-a9 \
		-cpu cortex-a9 \
		-nographic \
		-machine dumpdtb=virt.dtb
	dtc -I dtb -O dts -o virt.dts virt.dtb
	
linux:
	qemu-system-arm\
		-nographic \
		-kernel $(MMK_BIN) \
		-device loader,file=payloads/tinyLinux_qemu_vexpressa9_arm32.bin,addr=0x60600000 \
		-machine vexpress-a9 -cpu cortex-a9
	
linux_debug:
	tmux new-session -d \
	"echo '[qemu debug]' && qemu-system-arm -s -S \
                -machine vexpress-a9 -cpu cortex-a9 \
                -nographic \
                -kernel mmk_std.bin \
		        -device loader,file=zImage_std,addr=0x60800000 \
	" \
	&& tmux split-window -h "gdb-multiarch vmlinux_std -ex 'target remote localhost:1234'" \
	&& tmux -2 attach-session -d \
	&& tmux source-file ~/.tmux.conf

debug: $(MMK_BIN)
	cd $(SRC_PATH) && make tmp_debug
	
clean:
	rm -f $(MMK_BIN)
	cd $(SRC_PATH) && make clean
	cd $(PAYLOAD_PATH) && make clean

