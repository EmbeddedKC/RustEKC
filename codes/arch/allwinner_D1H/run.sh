# The first argument: binary image of MMK
# the second argument: binary image of Payload

export MMK_ENTRY_PA=0x40200000
export PAYLOAD_ENTRY_PA=0x40800000
export FIRMWARE_ENTRY_PA=0x40000000

sudo xfel ddr d1
sudo xfel write ${FIRMWARE_ENTRY_PA} ./opensbi_fw_jump_allwinner.bin
sudo xfel write ${MMK_ENTRY_PA} $1
sudo xfel write ${PAYLOAD_ENTRY_PA} $2
sudo xfel exec ${FIRMWARE_ENTRY_PA}
