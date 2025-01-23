.section .text.nktrampoline

.globl nk_gate

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

nk_gate:

    MOV x28, 0x0
    # LDR x28, [x28, #8]

    # let x28 to be PROXYCONTEXT
    PROXYCONTEXT x28

    LDR x29, [x28, #91*8] 
    # nkapi_level
    CMP x29, XZR
    B.NE nk_bypass 
    # nkapi_level is 0 means in os (gate), >0 means in mmk (bypass).

# addr=0xfffffffffffff014
nk_entry:

    # switch sie to disable interrupt

    # load OS's sie from proxy to x27
    # csrr x27, sie
    
    # write MMK's sie
    # LDR x27, [x28, #66*8]
    # csrw sie, x27
    
    ADD x29, x29, 1
    STR x29, [x28, #91*8]
    # api level add 1 when entry

    # load MMK's satp
    LDR x29, [x28, #64*8]

    UPDATE_PT x29

    # store OS's sie from x31 to proxy
    STR x27, [x28, #67*8]
    
    # store all outer kernel registers to proxy, 包括栈指针
    PROXYCONTEXT x27

    ADD x29, x27, #32*8
    SAVE_CTX_TO_ADDR x29
    # this macro would break x28 and x29

    # load nk sp
    LDR x29, [x27, #31*8]

    MOV SP, x29

    # let x28 to be APITABLE (METADATA + 0x800)
    MMKAPI_TABLE x28

    # the handler in nkcall vec: APITABLE + nk_call_num*8
    LSL x7, x7, #3
    ADD x28, x28, x7
    LDR x28, [x28, #0]
    # todo: LDR x28, [x28, x7]

    # handler function call
    BLR x28
    # addr=0xfffffffffffff0d4

    # let x28 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT x28
    
    # save ret value from function handler call
    STR x0, [x28, #32*8]
    STR x1, [x28, #33*8]

    # exit
    B nk_exit

.globl nk_bypass

nk_bypass:

    # let x28 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT x28

    ADD SP, SP,-32*8
    SAVE_CTX_TO_SP

    MMKAPI_TABLE x28
    # the handler in nkcall vec: MMKAPI_TABLE + nk_call_num*8
    LSL x7, x7, 3
    ADD x28, x28, x7
    LDR x28, [x28, #0]

    # handler function call
    BLR x28

    # let x28 to be ProxyContext (NK_TRAMPOLINE)
    PROXYCONTEXT x28

    LOAD_CTX_FROM_SP
    ADD sp,sp,32*8

    BR x30

    
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

    ## reached
    # save nk sp
    MOV x29, SP
    STR x29, [x28, #31*8]

    LDR x29, [x28, #91*8] 
    # nkapi level

    ADD x29, x29, -1
    STR x29, [x28, #91*8]
    # api level remove 1 when exit

    # load OS's satp
    LDR x27, [x28, #65*8]

    UPDATE_PT x27

    # enable interrupt
    # LDR x27, [x28, #67*8] # Load OS's sie
    # csrw sie, x27
    
    # load all outer kernel registers from proxy, 包括栈指针
    PROXYCONTEXT x29
    ADD x29, x29, #32*8
    LOAD_CTX_FROM_ADDR x29 
    
    # jump back, according to outer kernel's ra
    # nk_exit + 0xb0
    BR x30

    # t_breakpoint.
    # PROXYCONTEXT x28
    # LDR x27, [x28, #90*8]
    # BR x27
    # t_breakpoint_end.
