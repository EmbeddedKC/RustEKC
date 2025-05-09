# The first argument: elf of MMK
# The second argument: binary image of MMK
# the second argument: binary image of Payload

export BOOT_PA=0x40000000
export MMK_ENTRY_PA=0x40010000
export PAYLOAD_ENTRY_PA=0x40100000


tmux new-session -d \
"echo '[qemu debug]' && qemu-system-arm -s -S \
                -machine virt -nographic \
                -cpu cortex-a7 \
                -kernel $2
" \
&& tmux split-window -h "gdb-multiarch $1 -ex 'target remote localhost:1234'" \
&& tmux -2 attach-session -d
