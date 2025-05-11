### What is this tiny program?

After connecting to MMK, the address that the bootloader jumps to is no longer the RTOS entry, but the MMK entry.
When we need to run RTOS directly (without using MMK), in order not to change the address space, I added this small program. This tiny program is used to directly jump from MMK space to RTOS space. **You can put this program in the original location of MMK to assist in guiding RTOS execution.**


### Usage:

Open `param.h`, modify the macro `FW_JUMP_ADDR` to your RTOS entry. Then run:
```
make
```
Finally find the output `fw_jump.bin` or `fw_jump.elf` in `build` dictionary.
