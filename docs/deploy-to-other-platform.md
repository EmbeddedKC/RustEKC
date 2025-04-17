## Ultilize EKC for your own hardware platform
If you want to apply EKC to your own hardware platform (development board), please follow the steps below:

1. Copy a existing folder in `codes/arch`.

2. Reconfigure `BASE_ADDRESS` in `linker.ld`, set it to the entry address in your hardware platform.

3. Reconfigure the parameters in `src/config.rs`, let it suit your hardware platform.

4. Reconfigure the PTE bitflag in `src/pte.rs`, let it suit your hardware platform.

6. **If you are deploying to a new ISA**, choose the specific Rust toolchain.

6. **If you are deploying to a new ISA**, rewrite the function in `src/lib.rs`. Usually some `unsafe` inline assembly.

7. **If you are deploying to a new ISA**, re-implement `src/entry.asm`, `src/nk_gate.S` and `src/trap/trap.S`.






