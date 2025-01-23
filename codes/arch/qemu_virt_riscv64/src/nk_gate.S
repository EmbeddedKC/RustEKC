
.altmacro
.macro SAVE_REGISTER n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_REGISTER n
    ld x\n, \n*8(sp)
.endm

.macro SAVE_REGISTER2 n o
    sd x\n, \o*8+\n*8(x28)
.endm

.macro LOAD_REGISTER2 n o
    ld x\n, \o*8+\n*8(x28)
.endm


.section .text.nktrampoline

.globl nk_entry

    # 入参 x10 nkapi handler
    # 入参 x11~x15 params
    # 结构
    # #####################################
    # 高地址
    # 维护: outer satp
    # 加载: nk satp
    # 加载: outer register (0, 1, ......, 31)
    # 保存：nk register (0, 1, ......, 31)
    # 低地址
    # #####################################
nk_entry:

    # let x28 to be PROXYCONTEXT
    PROXYCONTEXT x28

    ld x30, 91*8(x28) # nkapi_level
    bne x30, zero, nk_bypass # nkapi_level is 0 means in os (gate), >0 means in mmk (bypass).

    # switch sie to disable interrupt
    PROXYCONTEXT x28
    
    # load OS's sie from proxy to x31
    csrr x31, sie
    
    # write MMK's sie
    ld x29, 66*8(x28)
    csrw sie, x29
    
    addi x30, x30, 1
    sd x30, 91*8(x28) # api level add 1 when entry

    # load MMK's satp
    ld x29, 64*8(x28)

    # 0xfffffffffffff022
    csrw satp, x29

    # store OS's sie from x31 to proxy
    sd x31, 67*8(x28)
    
    # store all outer kernel registers to proxy, 包括栈指针
    # TODO: no need to save ALL registers. 
    # Check calling convention and optimize it.
    .set n, 1
    .rept 27
       SAVE_REGISTER2 %n 32
        .set n, n+1
    .endr

    # load nk sp
    ld sp, 2*8(x28)

    # let x28 to be APITABLE (METADATA + 0x800)
    MMKAPI_TABLE x28

    # the handler in nkcall vec: APITABLE + nk_call_num*8
    sll x17, x17, 3
    add x28, x28, x17
    ld x28, 0(x28)

    # handler function call
    jalr x1, x28, 0

    # let x28 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT x28
    
    # save ret value from function handler call
    sd x10, (32+10)*8(x28)
    sd x11, (32+11)*8(x28)

    # exit
    j nk_exit


.globl nk_bypass

nk_bypass:
    # let x28 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT x28

    addi sp,sp,-30*8
    # store all nk registers
    .set n, 1
    .rept 28
        SAVE_REGISTER %n
        .set n, n+1
    .endr

    MMKAPI_TABLE x28
    # the handler in nkcall vec: MMKAPI_TABLE + nk_call_num*8
    sll x17, x17, 3
    add x28, x28, x17
    ld x28, 0(x28)

    # handler function call
    jalr x1, x28, 0

    # let x28 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT x28
    
    # keep the retval
    mv x29, x10
    mv x30, x11

    # load all nk registers
    .set n, 1
    .rept 28
        LOAD_REGISTER %n
        .set n, n+1
    .endr
    addi sp,sp,30*8

    # restore the retval
    mv x10, x29
    mv x11, x30

    jr x1

    
.globl nk_exit
    # 结构 
    # #####################################
    # 高地址
    # 加载: outer satp
    # 维护: nk satp
    # 加载: outer register (0, 1, ......, 31)
    # 保存：nk register (0, 1, ......, 31)
    # 低地址
    # #####################################
nk_exit:

    # let x28 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT x28

    # save MMK sp
    # No need to save other context in MMK~
    sd sp, 2*8(x28)

    ld x30, 91*8(x28) # nkapi level
    addi x30, x30, -1
    sd x30, 91*8(x28) # api level remove 1 when exit

    # load OS's satp
    ld x29, 65*8(x28)
    csrw satp, x29 
    # 0xfffffffffffff15e

    # enable interrupt
    ld x31, 67*8(x28) # Load OS's sie
    csrw sie, x31
    
    .set n, 1
    .rept 27
        LOAD_REGISTER2 %n 32
    .set n, n+1
    .endr

	
    # jump back, according to outer kernel's ra
    jr x1

