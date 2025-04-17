## Ultilize EKC for your own OS kernel
If you want to apply EKC to your own OS kernel, please follow the steps below:

1. Modify your linker file (`linker.ld`) and change the entry address to the jump address of EKC.

The entry address of EKC can be found in `config.rs`.

2. Add the provided EKC API library (available in this organization) in to your project.

If you are using Rust, add `mmi = { version = "0.1.9", git = "https://github.com/MemoryManagementKernel/mmi_rust.git"` to your `Cargo.toml` file.
If you are using C/C++, add `libekc/include` to your header file path, add `libekc/build/libmmk_arch.a` library when compiling.

3. Directly invoke EKC API in your initialization code (ususally `start.S` to assign privilege to your program's `.text`, `.data`, and `.bss` segments. You must complete the permission assignment in the first page of the OS, since EKC will only give you the read, write and execute permissions of the first page by default.

For example, you can use the following code in `entry.S` in aarch64:
```
    #include "mmkdef.h"

    MOV x3, MAP_IDENTICAL
	MOV x7, NKAPI_ALLOC

	MOV x0, 0
	LDR x1, =0x80800 //start physical page number
	LDR x2, =0x80880 //end physical page number
	MOV x4, MAP_PERM_R | MAP_PERM_W | MAP_PERM_X //PTE flag
	SUB x2, x2, x1

	MOV x28, ENTRY_GATE_ADDRESS
	BLR x28

    //the assembly code calls nkapi_alloc.
```


4. If your OS has a memory management module, decouple and remove it. All calls to the memory management module are changed to calls to the interface in EKC API library. This step may be more laborious because you need to restructure your OS code.

If your OS have no memory management module (e.g. Real-time OS), you can just skip this step.
If your OS have already decoupled memory management module (e.g. micro-kernel), this step won't be difficult.

5. If your OS has a trap handler module, just change the instrution written into IVTR(`stvec` in RISC-V or `VBAR` in Arm) to call the EKC API library to set the trap delegation address.

6. If the compilation goes well, you may get the binary file of your OS kernel. Put it somewhere easy to find, like `payload/your_own_OS.img`.

7. You will be able to change `PAYLOAD` in `Makefile` and try running the OS with EKC. In addition, you can use the security services provided in the EKC API.
