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

## Quick Start
We provide some compiled payload OS kernel in `payload/` for quick start. If you need to compile the payload yourself or view the source code of the payload, you can find the corresponding repository in this github organization.

Details can be found in `docs/quick-start.md`.

## Ultilize EKC for your own kernel/platform

More details can be found in `docs/deploy-to-other-platform.md` and `docs/deploy-to-other-kernel.md`.

## About us
If you have any comments, please send an email to us.

Yan_ice Email: [![](https://img.shields.io/badge/-yan2364728692@gmail.com-black?logo=gmail&style=flat)](mailto:yan2364728692@gmail.com)

JADDYK Email: [![](https://img.shields.io/badge/-jaddykwind@gmail.com-black?logo=gmail&style=flat)](mailto:jaddykwind@gmail.com)

## Note
**In this anonymous repository, the documents, the fel tools and other repositories in the organization were not put in.**


