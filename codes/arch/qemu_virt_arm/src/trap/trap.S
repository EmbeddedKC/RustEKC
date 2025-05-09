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

.macro MMK_HANDLER, kind
    SAVE_REGS
    mov     r0, sp
    mov     r1, #\kind
    bl      mmk_trap_handler
    RESTORE_REGS
    SUBS    PC, LR, #4
.endm

.p2align 11
.globl mmk_exception_vector_base
mmk_exception_vector_base:
    b reset_handler            // 0x00: Reset
    b undefined_handler        // 0x04: Undefined Instruction
    b svc_handler              // 0x08: Supervisor Call (SVC)
    b prefetch_abort_handler   // 0x0C: Prefetch Abort
    b data_abort_handler       // 0x10: Data Abort
    b reserved_handler         // 0x14: Reserved
    b irq_handler              // 0x18: IRQ
    b fiq_handler              // 0x1C: FIQ

// 以下是各个异常的处理函数（你可以定义为具体功能）

reset_handler:
    MMK_HANDLER 0
    b .

undefined_handler:
    MMK_HANDLER 1
    b .

svc_handler:
    MMK_HANDLER 2
    b .

prefetch_abort_handler:
    MMK_HANDLER 3
    b .

data_abort_handler:
    MMK_HANDLER 4
    b .

reserved_handler:
    MMK_HANDLER 5
    b .

irq_handler:
    MMK_HANDLER 6
    b .

fiq_handler:
    MMK_HANDLER 7
    b .
