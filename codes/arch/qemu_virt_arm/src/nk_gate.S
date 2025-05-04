.section .text.nktrampoline

.globl nk_gate

    # 入参 r10 nkapi handler
    # 入参 r11~r15 params
    # 结构
    # #####################################
    # 高地址
    # 维护: outer satp
    # 加载: nk satp
    # 加载: outer register (0, 1, ......, 31)
    # 保存：nk register (0, 1, ......, 31)
    # 低地址
    # #####################################

nk_gate:

    # let r10 to be PROXYCONTEXT
    PROXYCONTEXT r10

    LDR r11, [r10, #91*8] 
    # nkapi_level
    MOV r10, #0
    CMP r11, r10
    BNE nk_bypass 
    # nkapi_level is 0 means in os (gate), >0 means in mmk (bypass).

nk_entry:

    # load OS's CPSR (intr) from proxy to r7, keep r7 not changed!
    MRS r7, CPSR
    
    # disable interrupts
    BL disable_intr
    
    ADD r9, r9, 1
    STR r9, [r8, #91*8]
    # api level add 1 when entry

    # use MMK's satp
    LDR r0, [r8, #64*8]
    BL write_root_pt

    # store OS's CPSR (intr) from r7 to proxy
    STR r7, [r8, #67*8]
    
    # store all outer kernel registers to proxy, 包括栈指针
    PROXYCONTEXT r7
    # addr=0xfffffffffffff038
    ADD r9, r7, #32*8
    SAVE_CTX_TO_ADDR r9
    # this macro would break r8 and r9

    # load nk sp
    LDR r9, [r7, #31*8]
    # addr=0xfffffffffffff0c8

    MOV SP, r9

    # let r8 to be APITABLE (METADATA + 0x800)
    MMKAPI_TABLE r8
    # addr=0xfffffffffffff0d0

    # the handler in nkcall vec: APITABLE + nk_call_num*8
    LSL r7, r7, #3
    ADD r8, r8, r7
    LDR r8, [r8, #0]
    # todo: LDR r8, [r8, r7]

    # handler function call
    BLX r8
    # addr=0xfffffffffffff0e0

    # let r8 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT r8
    
    # save ret value from function handler call
    STR r0, [r8, #32*8]
    STR r1, [r8, #33*8]

    # exit
    B nk_exit

.globl nk_bypass

nk_bypass:

    # let r8 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT r8

    ADD SP, SP,-32*8
    SAVE_CTX_TO_SP

    MMKAPI_TABLE r8
    # the handler in nkcall vec: MMKAPI_TABLE + nk_call_num*8
    LSL r7, r7, 3
    ADD r8, r8, r7
    LDR r8, [r8, #0]

    # handler function call
    BLX r8

    # let r8 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT r8

    LOAD_CTX_FROM_SP
    ADD sp,sp,32*8

    BX r10

    
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

    # let r8 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT r8

    # save nk sp
    MOV r9, SP
    STR r9, [r8, #31*8]

    LDR r9, [r8, #91*8] 
    # nkapi level

    ADD r9, r9, -1
    STR r9, [r8, #91*8]
    # api level remove 1 when exit

    # load OS's satp
    LDR r0, [r8, #65*8]
    BL write_root_pt

    # tlbi alle1
    # dsb sy
    # isb

    # enable interrupt (Load OS's CPSR)
    # LDR r7, [r8, #67*8] 
    # MCR daif, r7

    LDR r7, [r8, #67*8]
    MSR cpsr, r7

    # BL enable_intr

    
    # load all outer kernel registers from proxy, 包括栈指针
    PROXYCONTEXT r9
    ADD r9, r9, #32*8
    LOAD_CTX_FROM_ADDR r9 
    
    # addr = -0xd38
    # jump back, according to outer kernel's ra
    BX r10

