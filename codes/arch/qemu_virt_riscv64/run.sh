# The first argument: binary image of MMK
# the second argument: binary image of Payload

export MMK_ENTRY_PA=0x80200000
export PAYLOAD_ENTRY_PA=0x80800000
export FIRMWARE_ENTRY_PA=0x80000000

echo "qemu starting..."
qemu-system-riscv64 \
                -machine virt -nographic \
                -bios ./opensbi_fw_jump_qemu.bin \
                -device loader,file=$1,addr=${MMK_ENTRY_PA} \
                -device loader,file=$2,addr=${PAYLOAD_ENTRY_PA} \
                -drive file=./fs.img,if=none,format=raw,id=x0 \
                -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0\
                -smp threads=2

