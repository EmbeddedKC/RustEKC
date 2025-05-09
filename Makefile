export cwd := $(shell pwd)

#########
# edited by user

export MODE ?= debug
export PAYLOAD ?= $(cwd)/payloads/freertos_blinky_qemu_aarch64.bin

export BOARD ?= qemu_virt_arm
# export BOARD ?= qemu_virt_riscv64
# export BOARD ?= allwinner_D1H
# export BOARD ?= qemu_virt_aarch64
# export BOARD ?= raspberry_pi4

export TARGET = armv7a-none-eabi
# export TARGET = aarch64-unknown-none
# export TARGET = riscv64gc-unknown-none-elf

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
		-M virt \
		-cpu cortex-a15 \
		-nographic \
		-machine dumpdtb=virt.dtb
	dtc -I dtb -O dts -o virt.dts virt.dtb
	
linux:
	qemu-system-arm\
		-M versatilepb \
		-nographic \
		-kernel zImage \
		-initrd initramfs.cpio.gz
	
debug: $(MMK_BIN)
	cd $(SRC_PATH) && make tmp_debug

clean:
	rm -f $(MMK_BIN)
	cd $(SRC_PATH) && make clean
	cd $(PAYLOAD_PATH) && make clean

