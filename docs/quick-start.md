## Quick start

We provide some compiled payload OS kernel in `payload/` for quick start. If you need to compile the payload yourself or view the source code of the payload, you can find the corresponding repository in this github organization.

0. Build the environment. This includes:

Find a Linux-based OS (Ubuntu 20.04 is recommended), install Rust nightly-2023-06-25 in [Rust](https://www.rust-lang.org/tools/install).

Install `qemu-system-riscv64`, `qemu-system-aarch64` or `qemu-system-arm32` on your computer, or prepare a development board such as `raspberry_pi4`, `Allwinner_Nezha_D1H`, `Kendryte_K210`.

*Note: currently we have found that the riscv64_qemu_virt architecture can only run on qemu version 4.2.0. Unknown problems may occur in higher QEMU versions, causing the system to get stuck.*

Install a toolchain, `riscv64-unknown-linux-musl-gcc`, `aarch64-none-elf-gcc-2021.07` or `arm-none-linux-gnueabi-gcc-4.3.2` is recommended.


1. Clone this repository, and edit the `Makefile` in repository root directory, and
change the following parameters:
`PAYLOAD` - Select a payload binary file for execution.
`BOARD` - Select a platform for building EKC. All the available platform can be found in `codes/arch`.
`TARGET` - select the rust toolchain, usually `aarch64-unknown-none`, `arm32-none-linux-gnueabi` or `riscv64gc-unknown-none-elf`.

1. Run the following code to check and establish the Rust environment for EKC.
``` shell
# check the environment
$ make env
```

1. If you want to run the demo in Qemu, just the following command:
``` shell
$ make && make test
```

1. To run in Allwinner D1-H development board, you need an external tool called [xfel](https://github.com/xboot/xfel). Use following command to install:
```
sudo apt install libusb-1.0-0-dev
git clone git@github.com:xboot/xfel
cd xfel
make
sudo make install
```

Use xfel:
Keep pressing the FEL button on board.
Connect the computer with type-C interface called *OTG* on board (the board would power on).
Release the FEL button. connect serial port (GND,RX,TX) to the computer.
Use PuTTY to access serial console, baudrate = 115200.
Run following codes:
```

xfel ddr d1
# check the status. Log message can be found in serial console.
xfel write 0x40000000 opensbi_xxx.bin
xfel write 0x40200000 MMK_xxx.bin
xfel write 0x40800000 payload_xxx.bin
# write the binary file
xfel exec 0x40000000
# execute the code.
```

5. To run in Raspberry Pi 4b or K210, some simple fel tools are available in `tools/`. Follow the instruction of `Makefile` in the corresponding fel tools. More details on this tools can be found in directory `docs`.