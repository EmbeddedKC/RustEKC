# The first argument: binary image of MMK
# the second argument: binary image of Payload

export MMK_ENTRY_PA=0x10000
export PAYLOAD_ENTRY_PA=0x100000


tmux new-session -d \
"echo '[qemu debug]' && qemu-system-arm -s -S \
                -machine virt -machine virtualization=on -nographic \
                -cpu cortex-a15 \
                -kernel $1 \
" \
&& tmux split-window -h "gdb-multiarch $1 -ex 'target remote localhost:1234'" \
&& tmux -2 attach-session -d
