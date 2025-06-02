.section .text

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

.align 12
.globl snktrampoline
.globl nk_gate

nk_gate:
snktrampoline:

    STMFD SP!, {R10-R12}

    # let r10 to be PROXYCONTEXT
    PROXYCONTEXT r10

    LDR r11, [r10, #PROXY_NKAPI_LEVEL] 
    # nkapi_level
    MOV r10, #0
    CMP r11, r10

    LDMFD SP!, {R10-R12}

    # fffff010
    BNE nk_bypass 
    # nkapi_level is 0 means in os (gate), >0 means in mmk (bypass).

nk_entry:
    #######################
    # Critical Note - calling convention:
    # r0 - r10 would be break when calling NKAPI.
    #######################

    # load OS's CPSR (intr) from proxy to r11, keep r11 not changed!
    MRS r10, CPSR

    # addr = 0x*1c
    # let r8 to be PROXYCONTEXT
    PROXYCONTEXT r8
    
    # disable interrupts
    DISABIE_INTR

    # nkapi_level
    LDR r9, [r8, #PROXY_NKAPI_LEVEL] 
    ADD r9, r9, #1
    STR r9, [r8, #PROXY_NKAPI_LEVEL]
    # api level add 1 when entry

    # use MMK's satp
    LDR r9, [r8, #PROXY_EKC_SATP]

    # addr = 0xfffff038
    WRITE_ROOT_PT r9

    # store OS's CPSR (intr) from r11 to proxy
    # now r10 can be used.
    STR r10, [r8, #PROXY_OS_CPSR]
    
    # store all outer kernel registers to proxy, 包括栈指针

    ADD r9, r8, #(PROXY_OS_REGS)
    SAVE_CTX_TO_ADDR r9

    # load nk sp
    LDR r9, [r8, #PROXY_EKC_SP]

    # addr = 0x*08c
    MOV SP, r9    

    # let r8 to be APITABLE (METADATA + 0x800)
    MMKAPI_TABLE r8

    # the handler in nkcall vec: APITABLE + nk_call_num*8
    LSL r7, r7, #3
    ADD r8, r8, r7
    LDR r8, [r8, #0]
    # todo: LDR r8, [r8, r7]

    # handler function call

    # params more than 4 should be in SP.
    # caller-cleaned stack.
    push {r4, r5}
    BLX r8  
    pop {r4, r5}
    # blx addr=0x*0a4

    # let r8 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT r8
    
    # save ret value from function handler call
    STR r0, [r8, #PROXY_OS_REGS]
    STR r1, [r8, #PROXY_OS_REGS+4]

    # exit
    B nk_exit

.globl nk_bypass

nk_bypass:
    // addr = 0x*0b0
    SAVE_REGS

    MMKAPI_TABLE r8
    //the handler in nkcall vec: MMKAPI_TABLE + nk_call_num*8
    //MMK API always use 8 byte per vector.
    LSL r7, r7, #3
    ADD r8, r8, r7
    LDR r8, [r8, #0]
    //0xfffff0cc

    # params more than 4 should be in SP.
    # caller-cleaned stack.

    push {r4, r5}
    # handler function call
    BLX r8
    pop {r4, r5}

    //update the return value
    //Note: stack struct = (low) lr, CPSR, r12...r0 (high)
    STR r0, [sp, #2*4]
    STR r1, [sp, #3*4]

    RESTORE_REGS

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
    # 0x*0fc
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
    LDR r9, [r8, #PROXY_OS_SATP]
    # addr = 0xfffff118
    WRITE_ROOT_PT r9

    # tlbi alle1
    # dsb sy
    # isb

    ADD r8, r8, #PROXY_OS_REGS
    LOAD_CTX_FROM_ADDR r8
    
    # enable interrupt (Load OS's CPSR)
    PROXYCONTEXT r8
    LDR r8, [r8, #PROXY_OS_CPSR]
    MSR cpsr_c, r8

    # addr = 0x*170
    # jump back, according to outer kernel's ra
    BX lr

.ltorg

.align 12