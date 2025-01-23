# The first argument: binary image of MMK
# the second argument: binary image of Payload

export MMK_ENTRY_PA=0x80200000
export PAYLOAD_ENTRY_PA=0x80800000
export FIRMWARE_ENTRY_PA=0x80000000


tmux new-session -d \
"echo '[qemu session]' && ./qemu-system-riscv64-4.2.0 -s -S \
		-bios ./opensbi_fw_jump_qemu.bin \
                -machine virt -nographic \
		-device loader,file=$2,addr=${MMK_ENTRY_PA} \
		-device loader,file=$3,addr=${PAYLOAD_ENTRY_PA} \
" \
&& tmux split-window -h "gdb-multiarch $1 -ex 'target remote localhost:1234'" \
&& tmux -2 attach-session -d
