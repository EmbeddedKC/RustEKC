BUILD_DIR       = build

CROSS   := arm-none-eabi-
CC      = $(CROSS)gcc
OBJCOPY = $(CROSS)objcopy
ARCH    = $(CROSS)ar

# Generate GCC_VERSION in number format
GCC_VERSION = $(shell $(CC) --version | grep ^$(CC) | sed 's/^.* //g' | awk -F. '{ printf("%d%02d%02d"), $$1, $$2, $$3 }')
GCC_VERSION_NEED_ZICSR = "110100"

CPPFLAGS = \
	-fno-builtin -fno-builtin-function -nostdlib \
	-mgeneral-regs-only -I . -O0

CFLAGS  = -Wall -fno-builtin -fno-builtin-function -nostdlib \
	-fmessage-length=0 -fno-common -mgeneral-regs-only \
	-ffunction-sections -fdata-sections -O0

LDFLAGS = -nostartfiles -Tfake_rom.lds -fno-builtin -nostdlib \
	-Xlinker --gc-sections -fno-common -mgeneral-regs-only

SRCS = main.c uart.c my_stdio.c

ASMS = start.S

OBJS = $(SRCS:%.c=$(BUILD_DIR)/%.o) $(ASMS:%.S=$(BUILD_DIR)/%.o)
DEPS = $(SRCS:%.c=$(BUILD_DIR)/%.d) $(ASMS:%.S=$(BUILD_DIR)/%.d)

$(BUILD_DIR)/fw_jump.bin: $(BUILD_DIR)/fw_jump.elf
	$(OBJCOPY) $< -O binary $(BUILD_DIR)/fw_jump.bin

$(BUILD_DIR)/fw_jump.elf: $(OBJS) fake_rom.lds Makefile
	$(CC) $(LDFLAGS) $(OBJS) -o $@

$(BUILD_DIR)/%.o: %.c Makefile
	@mkdir -p $(@D)
	$(CC) $(CPPFLAGS) $(CFLAGS) -MMD -MP -c $< -o $@

$(BUILD_DIR)/%.o: %.S Makefile
	@mkdir -p $(@D)
	$(CC) $(CPPFLAGS) $(CFLAGS) -MMD -MP -c $< -o $@

clean:
	rm -rf $(BUILD_DIR)
