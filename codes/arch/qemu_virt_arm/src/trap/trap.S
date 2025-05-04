.macro SAVE_REGS
    STMFD SP!, {R0-R14}
    mrs r10, CPSR
    STMFD SP!, {r10}
.endm

.macro RESTORE_REGS
    STMFD SP!, {r10}
    msr CPSR, r10
    STMFD SP!, {R0-R14}
.endm

.macro INVALID_EXCP, kind, source
.p2align 7
    LDR r8, =-0x2200
    MOV sp, r8
    SAVE_REGS
    mov     r0, sp
    mov     r1, #\kind
    mov     r2, #\source
    bl      invalid_exception 
    # bl: branch to addr, and save pc+4 to lr(x30). like jal.
    b       .Lexception_return
.endm

.macro HANDLE_SYNC
.p2align 7
    SAVE_REGS
    mov     r0, sp
    bl      handle_sync_exception
    b       .Lexception_return
.endm

.macro HANDLE_IRQ
.p2align 7
    SAVE_REGS
    mov     r0, sp
    bl      handle_irq_exception
    b       .Lexception_return
.endm

.macro MMK_HANDLER, kind, source
.p2align 7
    LDR r8, =-0x2200
    MOV sp, r8
    SAVE_REGS
    mov     r0, sp
    mov     r1, #\kind
    mov     r2, #\source
    bl      mmk_trap_handler
    b       .Lexception_return
.endm

.section .text
.p2align 11
.globl os_exception_vector_base
os_exception_vector_base:
    // current EL, with SP_EL0
    INVALID_EXCP 0 0
    INVALID_EXCP 1 0
    INVALID_EXCP 2 0
    INVALID_EXCP 3 0

    // current EL, with SP_ELx
    HANDLE_SYNC
    HANDLE_IRQ
    INVALID_EXCP 2 1
    INVALID_EXCP 3 1

    // lower EL, aarch64
    HANDLE_SYNC
    HANDLE_IRQ
    INVALID_EXCP 2 2
    INVALID_EXCP 3 2

    // lower EL, aarch32
    INVALID_EXCP 0 3
    INVALID_EXCP 1 3
    INVALID_EXCP 2 3
    INVALID_EXCP 3 3

.p2align 11
.globl mmk_exception_vector_base
mmk_exception_vector_base:
    // current EL, with SP_EL0
    INVALID_EXCP 0 0
    INVALID_EXCP 1 0
    INVALID_EXCP 2 0
    INVALID_EXCP 3 0

    // current EL, with SP_ELx
    MMK_HANDLER 0 1
    MMK_HANDLER 1 1
    MMK_HANDLER 2 1
    MMK_HANDLER 3 1

    // lower EL, aarch64
    MMK_HANDLER 0 2
    MMK_HANDLER 1 2
    INVALID_EXCP 2 2
    INVALID_EXCP 3 2

    // lower EL, aarch32
    INVALID_EXCP 0 3
    INVALID_EXCP 1 3
    INVALID_EXCP 2 3
    INVALID_EXCP 3 3

.Lexception_return:
    RESTORE_REGS
    
    //eret
    SUBS    PC, LR, #4
