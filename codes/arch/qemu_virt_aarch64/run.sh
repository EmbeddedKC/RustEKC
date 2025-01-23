# The first argument: binary image of MMK
# the second argument: binary image of Payload

export MMK_ENTRY_PA=0x40000000
export PAYLOAD_ENTRY_PA=0x40400000

echo "qemu starting..."
qemu-system-aarch64 \
                -nographic \
                -cpu cortex-a72 \
                -machine virt -machine virtualization=on \
                -kernel $1 \
                -device loader,file=$2,addr=${PAYLOAD_ENTRY_PA} \
                # -drive file=fs.img,if=none,format=raw,id=x0 \
                # -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
                # -bios ./opensbi_fw_jump_qemu.bin \
                # -device loader,file=$2,addr=${PAYLOAD_ENTRY_PA} \

