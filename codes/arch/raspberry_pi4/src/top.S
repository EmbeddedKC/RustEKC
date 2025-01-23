.altmacro
.macro PROXYCONTEXT n
    MOV \n, -0x3000
.endm
.altmacro
.macro MMKAPI_TABLE n
    MOV \n, -0x3000+0x800
.endm
.altmacro
.macro CONFIG_DATA n
    MOV \n, -0x3000+0x400
.endm

.altmacro
.macro SAVE_STACK r s
    STR x\r, [\s, #\r*8]
.endm

.macro LOAD_STACK r s
    LDR x\r, [\s, #\r*8]
.endm

.macro UPDATE_PT r
    # nk_exit + 0x1c
    MSR TTBR0_EL1, \r
    MSR TTBR1_EL1, \r

    # flush TLB, (may has problems here!)
    MOV \r, 0
    tlbi aside1, \r
    dsb sy
    isb
.endm

.macro SAVE_CTX_TO_SP
    # save x0 ~ x31
    .set n, 0
    .rept 31
        SAVE_STACK %n SP
        # STR [x%n, SP, #%n*8]
        .set n, n+1
    .endr
    # save sp as x32
    MOV x29, SP
    STR X29, [SP, #31*8]
.endm

.macro LOAD_CTX_FROM_SP
    # save x0 ~ x31
    .set n, 0
    .rept 31
        LOAD_STACK %n SP
        # LDR x%n, [SP, #%n*8]
        .set n, n+1
    .endr
    # load sp as x32
    LDR x29, [SP, #31*8]
    MOV SP, x29
.endm

.macro SAVE_CTX_TO_ADDR a
    MOV x29, \a

    # save x0 ~ x30
    .set n, 0
    .rept 31
        SAVE_STACK %n x29
        # STR x%n, [x29, #%n*8]
        .set n, n+1
    .endr

    # save sp as x31
    MOV x28, SP
    STR x28, [x29, #31*8]
.endm

.macro LOAD_CTX_FROM_ADDR a
    MOV x29, \a

    # load x0 ~ x28
    .set n, 0
    .rept 29
        LOAD_STACK %n x29
        # LDR x%n, [x29, #%n*8]
        .set n, n+1
    .endr
    # load x30
    LDR x30, [x29, #30*8]

    # load sp as x31
    LDR x28, [x29, #31*8]
    MOV SP, x28
.endm

.include "src/cpu.S"
.include "src/nk_gate.S"
.include "src/trap/trap.S"

# .include "src/trap/trap_signal.S"
