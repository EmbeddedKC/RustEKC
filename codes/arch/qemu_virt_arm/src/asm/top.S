#define USIZE_WIDTH 4
#define USIZE_BIT 2


// PROXYCONTEXT.

.equ PROXY_EKC_REGS,      (0*USIZE_WIDTH)
.equ PROXY_EKC_SP,        (13*USIZE_WIDTH)

.equ PROXY_OS_REGS,       (32*USIZE_WIDTH)
.equ PROXY_OS_SP,         ((32+13)*USIZE_WIDTH)

.equ PROXY_EKC_SATP,      (64*USIZE_WIDTH)
.equ PROXY_OS_SATP,       (65*USIZE_WIDTH)
.equ PROXY_EKC_CPSR,       (66*USIZE_WIDTH)
.equ PROXY_OS_CPSR,       (67*USIZE_WIDTH)

.equ PROXY_NKAPI_LEVEL,   (91*USIZE_WIDTH)

//.equ PROXY_OS_REGS, 32*USIZE_WIDTH

.altmacro
.macro PROXYCONTEXT n
    LDR \n, =-0x3000
.endm
.altmacro
.macro MMKAPI_TABLE n
    LDR \n, =-0x3000+0x800
.endm
.altmacro
.macro CONFIG_DATA n
    LDR \n, =-0x3000+0x400
.endm

.altmacro
.macro SAVE_STACK z s
    STR r\z, [\s, #\z*USIZE_WIDTH]
.endm

.macro LOAD_STACK z s
    LDR r\z, [\s, #\z*USIZE_WIDTH]
.endm

.macro SAVE_REGS
    STMFD SP!, {R0-R12}
    mrs r10, CPSR
    STMFD SP!, {r10, r14}
.endm

.macro RESTORE_REGS
    LDMFD SP!, {r10, r14}
    msr CPSR, r10
    LDMFD SP!, {R0-R12}
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

.include "src/asm/entry.S"
.include "src/asm/cpu.S"
.include "src/asm/nk_gate.S"
.include "src/trap/trap.S"
# .include "src/trap/trap_signal.S"
