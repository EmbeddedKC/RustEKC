# The first argument: binary image of MMK
# the second argument: binary image of Payload

export MMK_ENTRY_PA=0x80020000
export PAYLOAD_ENTRY_PA=0x80200000
export FIRMWARE_ENTRY_PA=0x80000000

export K210_BURNER=./tools/kflash.py
export PORT=/dev/ttyUSB0

export SBI_SIZE=131072 #0x20000
export MMK_SIZE=2097152 #0x200000

#build image
cp ./opensbi_fw_jump_k210.bin img.bin
dd if=$1 of=./img.bin bs=${SBI_SIZE} seek=1
dd if=$2 of=./img.bin bs=${MMK_SIZE} seek=1

#fetch tool
(which ${K210_BURNER}) || (git clone https://github.com/sipeed/kflash.py.git && mv kflash.py tools)

#burn to K210
sudo chmod 777 ${PORT}
python3 ${K210_BURNER} -p ${PORT} -b 1500000 ${OUTPUT}
python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct ${PORT} 115200

