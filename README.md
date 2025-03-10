# Rust EKC

## Description
**Embedded Kernel Compartment** is a portable kernel compartment that eliminates the necessity for complexities such as virtualization, extended ISAs, and secure hardware
modules. Considering the kernel grows in size and complexity and the absence
of cross-platform isolation mechanisms, we hope this design would enhance operating system securitysolve this problem.

This reporitory is the cross-platform prototype of EKC implemented in **Rust** with good portability and reusability, and we prepared some compiled payloads in this repository as examples.

## Environment
- [Rust](https://www.rust-lang.org/tools/install): nightly-2023-06-25

RISC-V64 ISAs environment:
- [qemu](https://github.com/qemu/qemu): qemu-system-riscv64 4.2.1
- target: riscv64gc-unknown-none-elf
- toolchain: [riscv64-unknown-linux-musl-gcc](https://github.com/riscv-collab/riscv-gnu-toolchain)

AArch64 ISAs environment:
- [qemu](https://github.com/qemu/qemu): qemu-system-aarch64 4.2.1
- target: aarch64-none-elf
- toolchain: [aarch64-none-elf-gcc-2021.07](https://developer.arm.com/downloads/-/gnu-a/10-3-2021-07)

## Repository structure
`build` contains all the output files in building RustEKC.
`codes` contains the source code (Rust) of RustEKC.
`docs` contains the documentation of RustEKC.
`lib` contains a small wrapper for EKC API implemented in C, which can be used by payloads.
`payloads` contains some available binary file of demo payloads. The source code can be found in other repositories of this organization.
`tools` contains some useful ultilities for testing and evaluating RustEKC.

``

## Quick start

We provide some compiled payload OS kernel in `payload/` for quick start. If you need to compile the payload yourself or view the source code of the payload, you can find the corresponding repository in this github organization.

0. Build the environment. This includes:

Find a Linux-based OS (Ubuntu 20.04 is recommended), install Rust nightly-2023-06-25 in [Rust](https://www.rust-lang.org/tools/install).

Install `qemu-system-riscv64` or `qemu-system-aarch64` on your computer, or prepare a development board such as `raspberry_pi4`, `Allwinner_Nezha_D1H`, `Kendryte_K210`.

Install a toolchain, `riscv64-unknown-linux-musl-gcc` or `aarch64-none-elf-gcc-2021.07` is recommended.


1. Clone this repository, and edit the `Makefile` in repository root directory, and
change the following parameters:
`PAYLOAD` - Select a payload binary file for execution.
`BOARD` - Select a platform for building EKC. All the available platform can be found in `codes/arch`.
`TARGET` - select the rust toolchain, usually `aarch64-unknown-none` or `riscv64gc-unknown-none-elf`.

2. Run the following code to check and establish the Rust environment for EKC.
``` shell
# check the environment
$ make env
```

3. If you want to run the demo in Qemu, just the following command:
``` shell
$ make && make test
```

4. To run in Allwinner D1-H development board, you need an external tool called [xfel](https://github.com/xboot/xfel). Use following command to install:
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

## Ultilize EKC for your own OS kernel
If you want to apply EKC to your own OS kernel, please follow the steps below:

1. Modify your linker file and change the link base address to the jump address of EKC.

2. Add the provided EKC API library (available in this organization) in to your project.

3. Directly use EKC API in your initialization code (ususally `start.S` to assign privilege to your program's `.text`, `.data`, and `.bss` segments. You must complete the permission assignment in the first page of the OS, since EKC will only give you the read, write and execute permissions of the first page by default.

4. If your OS has a memory management module, remove it. All calls to the memory management module are changed to calls to the interface in EKC API library. This step may be more laborious because you need to restructure your OS code.

5. If your OS has a trap handler module, just change the instrution written into IVTR(`stvec` in RISC-V or `VBAR` in Arm) to call the EKC API library to set the trap delegation address.

6. If the compilation goes well, you may get the binary file of your OS kernel. Put it somewhere easy to find, like `payload/`

7. You will be able to change `PAYLOAD` in `Makefile` and try running the OS with EKC. In addition, you can use the security services provided in the EKC API.

More details can be found in `docs/`.

## About us
If you have any comments, please send an email to us.

Yan_ice Email: [![](https://img.shields.io/badge/-yan2364728692@gmail.com-black?logo=gmail&style=flat)](mailto:yan2364728692@gmail.com)

JADDYK Email: [![](https://img.shields.io/badge/-jaddykwind@gmail.com-black?logo=gmail&style=flat)](mailto:jaddykwind@gmail.com)

## Note
**In this anonymous repository, the documents, the fel tools and other repositories in the organization were not put in.**


