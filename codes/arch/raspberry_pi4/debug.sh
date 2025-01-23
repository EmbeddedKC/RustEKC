# The first argument: binary image of MMK
# the second argument: binary image of Payload

export MMK_ENTRY_PA=0x80000000
export PAYLOAD_ENTRY_PA=0x80800000
export FIRMWARE_ENTRY_PA=0x80000000


tmux new-session -d \
"echo '[qemu session]' && qemu-system-aarch64 -s -S \
                -machine virt -nographic \
                -cpu cortex-a72 \
                -kernel $2
" \
&& tmux split-window -h "gdb-multiarch $1 -ex 'target remote localhost:1234'" \
&& tmux -2 attach-session -d
