# The first argument: binary image of MMK
# the second argument: binary image of Payload

export BOOT_PA=0x60000000
export MMK_ENTRY_PA=0x60010000
export PAYLOAD_ENTRY_PA=0x60600000

echo "qemu starting..."
qemu-system-arm \
                -nographic \
                -machine vexpress-a9 -cpu cortex-a9 \
                -kernel $1\
                -device loader,file=$2,addr=${PAYLOAD_ENTRY_PA} \
                # -initrd initramfs.cpio.gz
                # -device loader,file=$1,addr=${MMK_ENTRY_PA}
                # -device loader,file=$2,addr=${PAYLOAD_ENTRY_PA} \
                # -drive file=fs.img,if=none,format=raw,id=x0 \
                # -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
                # -bios ./opensbi_fw_jump_qemu.bin \
                # -device loader,file=$2,addr=${PAYLOAD_ENTRY_PA} \

#################

# 上回说到：vexpressA9下mmk能跑但是不输出东西，可能找错pl011的MMIO接口了

###############
