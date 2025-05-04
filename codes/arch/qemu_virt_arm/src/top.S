.altmacro
.macro PROXYCONTEXT n
    LDR \n, =-0x1000
.endm
.altmacro
.macro MMKAPI_TABLE n
    LDR \n, =-0x1000+0x800
.endm
.altmacro
.macro CONFIG_DATA n
    LDR \n, =-0x1000+0x400
.endm

.altmacro
.macro SAVE_STACK r s
    STR r\r, [\s, #\r*8]
.endm

.macro LOAD_STACK r s
    LDR r\r, [\s, #\r*8]
.endm

.macro SAVE_CTX_TO_SP
    # save r0 ~ r14
    .set n, 0
    .rept 15
        SAVE_STACK %n SP
        # STR [r%n, SP, #%n*8]
        .set n, n+1
    .endr
    # save sp as x32
    MOV r9, SP
    STR r9, [SP, #31*8]
.endm

.macro LOAD_CTX_FROM_SP
    # save r0 ~ r14
    .set n, 0
    .rept 15
        LOAD_STACK %n SP
        # LDR r%n, [SP, #%n*8]
        .set n, n+1
    .endr
    # load sp as x32
    LDR r9, [SP, #31*8]
    MOV SP, r9
.endm

.macro SAVE_CTX_TO_ADDR a
    MOV r9, \a

    # save r0 ~ r10
    .set n, 0
    .rept 15
        SAVE_STACK %n r9
        # STR r%n, [r9, #%n*8]
        .set n, n+1
    .endr

    # save sp as r11
    MOV r8, SP
    STR r8, [r9, #31*8]
.endm

.macro LOAD_CTX_FROM_ADDR a
    MOV r9, \a

    # load r0 ~ r8
    .set n, 0
    .rept 15
        LOAD_STACK %n r9
        # LDR r%n, [r9, #%n*8]
        .set n, n+1
    .endr
    # load r10
    LDR r10, [r9, #30*8]

    # load sp as r11
    LDR r8, [r9, #31*8]
    MOV SP, r8
.endm

.include "src/cpu.S"
.include "src/nk_gate.S"
.include "src/trap/trap.S"
# .include "src/trap/trap_signal.S"
