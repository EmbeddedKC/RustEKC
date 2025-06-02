# Building

KERNEL_ELF := $(shell pwd)/target/$(TARGET)/$(MODE)/MMK
KERNEL_BIN := $(KERNEL_ELF)_$(BOARD).bin

DISASM_TMP := target/$(TARGET)/$(MODE)/asm

ifeq ($(MODE), release)
	FLAG := --release
endif

ARCH_PATH := ../arch/$(BOARD)

SDCARD := /dev/sdb
APPS := ../user/src/bin/*
TOP := ../../k210.bin


DISASM ?= -x

build: $(KERNEL_BIN)
	@cp $(KERNEL_BIN) $(OUTPUT_PATH)/
	@cp $(KERNEL_ELF) $(OUTPUT_PATH)/

env:
	rustup override set nightly-2023-06-25
	- (rustup target list | grep "$(TARGET) (installed)") || rustup target add $(TARGET)
	@echo ====================
	rustup component add rust-src
	rustup component add llvm-tools-preview

gen_toml: 
	@cat Cargo.toml.template > Cargo.toml
	@echo "mmk_arch = {path = \"$(ARCH_PATH)\"}" >> Cargo.toml
# dev/zero永远输出0
sdcard: 
	@echo "Are you sure write to $(SDCARD) ? [y/N] " && read ans && [ $${ans:-N} = y ]
	@sudo dd if=/dev/zero of=$(SDCARD) bs=1048576 count=16
	@sudo dd if=$(FS_IMG) of=$(SDCARD)

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@
#	@$(OBJDUMP) -S $(KERNEL_ELF) > $(KERNEL_ELF).S


kernel: env gen_toml
	@echo Platform: $(BOARD)
	RUSTFLAGS='-Clink-arg=-T$(ARCH_PATH)/linker.ld -Ctarget-feature=-neon -Cforce-frame-pointers=yes' \
	cargo build $(FLAG) --target $(TARGET) -Zbuild-std=core,alloc
	@echo kernel build over

tmp_run: build
	cd $(ARCH_PATH) && sudo sh run.sh $(KERNEL_BIN) $(PAYLOAD_BIN)

tmp_debug: build
	cd $(ARCH_PATH) && sudo sh debug.sh $(KERNEL_ELF) $(KERNEL_BIN) $(PAYLOAD_BIN) $(PAYLOAD_ELF) 

clean:
	@cargo clean
