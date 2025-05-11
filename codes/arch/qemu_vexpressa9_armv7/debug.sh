# The first argument: elf of MMK
# The second argument: binary image of MMK
# the second argument: binary image of Payload

export BOOT_PA=0x60000000
export MMK_ENTRY_PA=0x60010000
export PAYLOAD_ENTRY_PA=0x60600000


tmux new-session -d \
"echo '[qemu debug]' && qemu-system-arm -s -S \
                -machine vexpress-a9 -cpu cortex-a9 \
                -nographic \
                -kernel $2 \
		        -device loader,file=$3,addr=${PAYLOAD_ENTRY_PA} \
" \
&& tmux split-window -h "gdb-multiarch $1 -ex 'target remote localhost:1234'" \
&& tmux -2 attach-session -d
