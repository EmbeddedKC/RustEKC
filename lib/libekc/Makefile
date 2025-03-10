# Toolchain

# Please edit it: "aarch64" or "riscv64".
ARCH := aarch64 

# Please edit it: let it be a vailable toolchain in your computer.
CHAIN := $(ARCH)-linux-gnu


export CC = $(CHAIN)-gcc
export LD = $(CHAIN)-ld
export AR = $(CHAIN)-ar

# Directories
WORK_DIR = $(shell pwd)
export BUILD_DIR = $(WORK_DIR)/build

$(shell mkdir -p $(BUILD_DIR))

# Source
SRC = $(wildcard src/*.c)
OBJ = $(addsuffix .o, $(basename $(SRC)))

# Flags
INCFLAGS := -I$(WORK_DIR)/include

CFLAGS = -fno-pic -static -O2 -Wall $(INCFLAGS) \
         -fno-builtin -fno-stack-protector -ffreestanding -Wno-main

# Build target
LIB = $(BUILD_DIR)/libmmk_$(ARCH).a

all: $(LIB)

$(LIB): $(OBJ)
	$(AR) cr $@ $^

