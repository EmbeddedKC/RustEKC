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

    STMFD SP!, {R10-R12}

    # let r10 to be PROXYCONTEXT
    PROXYCONTEXT r10

    LDR r11, [r10, #91*4] 
    # nkapi_level
    MOV r10, #0
    CMP r11, r10

    LDMFD SP!, {R10-R12}

    // fffff010
    BNE nk_bypass 
    # nkapi_level is 0 means in os (gate), >0 means in mmk (bypass).

nk_entry:

    # let r10 to be PROXYCONTEXT
    PROXYCONTEXT r10

    # load OS's CPSR (intr) from proxy to r7, keep r7 not changed!
    MRS r7, CPSR
    
    # disable interrupts
    BL disable_intr
    
    ADD r9, r9, #1
    STR r9, [r8, #PROXY_NKAPI_LEVEL]
    # api level add 1 when entry

    # use MMK's satp
    LDR r0, [r8, #PROXY_EKC_SATP]
    BL write_root_pt

    # store OS's CPSR (intr) from r7 to proxy
    STR r7, [r8, #PROXY_OS_CPSR]
    
    # store all outer kernel registers to proxy, 包括栈指针
    PROXYCONTEXT r7
    # addr=0xfffffffffffff038
    ADD r9, r7, #(PROXY_OS_REGS)
    SAVE_CTX_TO_ADDR r9
    # this macro would break r8 and r9

    # load nk sp
    LDR r9, [r7, #PROXY_EKC_SP]
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
    STR r0, [r8, #PROXY_OS_REGS]
    STR r1, [r8, #PROXY_OS_REGS+4]

    # exit
    B nk_exit

.globl nk_bypass

nk_bypass:

    SAVE_REGS

    MMKAPI_TABLE r8
    //the handler in nkcall vec: MMKAPI_TABLE + nk_call_num*8
    //MMK API always use 8 byte per vector.
    LSL r7, r7, #3
    ADD r8, r8, r7
    LDR r8, [r8, #0]
    //0xfffff0d4
    # handler function call
    BLX r8

    //update the return value
    //Note: stack struct = CPSR, lr, r0, r1...r12
    STR r0, [sp, #8]
    STR r1, [sp, #8]

    RESTORE_REGS

    //0xfffff0ec Note============Here the LR is not the correct LR. Find why.
    BX LR

    
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
    STR r9, [r8, #PROXY_EKC_SP]

    LDR r9, [r8, #PROXY_NKAPI_LEVEL] 
    # nkapi level

    ADD r9, r9, #-1
    STR r9, [r8, #PROXY_NKAPI_LEVEL]
    # api level remove 1 when exit

    # load OS's satp
    LDR r0, [r8, #PROXY_OS_SATP]
    BL write_root_pt

    # tlbi alle1
    # dsb sy
    # isb

    # enable interrupt (Load OS's CPSR)
    LDR r7, [r8, #PROXY_OS_CPSR]
    MSR cpsr, r7

    # BL enable_intr

    
    # load all outer kernel registers from proxy, 包括栈指针
    PROXYCONTEXT r9
    ADD r9, r9, #PROXY_OS_REGS
    LOAD_CTX_FROM_ADDR r9 
    
    # addr = -0xd38
    # jump back, according to outer kernel's ra
    BX lr

