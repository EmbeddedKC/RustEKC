PAGE_SIZE := 4096 # 0x1000
# MMK_SIZE ?= 2097152 # 0x200000
MMK_SIZE ?= 4194304 # 0x400000

MMK_BIN ?= ./MMK_raspberry_pi4.bin
PAYLOAD_BIN ?= ./RTOSDemo.bin

SD_DEV := /dev/sdb1

MODE := 1

OUTPUT := ./pi4b/kernel8.img

build: clean sdcard/config.txt update umount

$(OUTPUT):
ifeq ($(MODE),1)
	cp $(MMK_BIN) $(OUTPUT)
	dd if=$(PAYLOAD_BIN) of=$(OUTPUT) bs=$(MMK_SIZE) seek=1
else
	cp $(PAYLOAD_BIN) $(OUTPUT)
endif

sdcard/config.txt: 
	mkdir -p sdcard
	sudo mount $(SD_DEV) sdcard
	sudo rm -rf sdcard/*

mount: sdcard/config.txt

update: $(OUTPUT)
	sudo cp pi4b/* sdcard/ -r

umount: sdcard
	sudo umount sdcard
	rm -rf sdcard

clean:
	rm -f $(OUTPUT) 
