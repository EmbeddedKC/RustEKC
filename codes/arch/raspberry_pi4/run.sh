# The first argument: binary image of MMK
# the second argument: binary image of Payload

export MMK_ENTRY_PA=0x40000000
export PAYLOAD_ENTRY_PA=0x40400000

echo "..."
cp $1 /home/yanice/Desktop/AARCH64_QEMU_FreeRTOS/tools/pi4b_fel_tool/
cd /home/yanice/Desktop/AARCH64_QEMU_FreeRTOS/tools/pi4b_fel_tool && make