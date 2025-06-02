.section .text
////////////////////////////
// Assuming that OS has initialized all the stack pointer.
////////////////////////////


// Require IRQ: sp point to a small memory space.
.macro RECORD_IRQ a
    str	    r0, [sp, #0]
    str	    r7, [sp, #4]
    str	    r8, [sp, #8]
    mov r8, #\a
.endm

.macro RESTORE_IRQ
    ldr	    r0, [sp, #0]
    ldr	    r7, [sp, #4]
    ldr	    r8, [sp, #8]
.endm

.globl mmk_exception_vector_base
.globl strampoline
.align 12
strampoline:
mmk_exception_vector_base:
_reset_handler:
    b reset_handler            // 0x00: Reset
_undefined_handler:
    b undefined_handler        // 0x04: Undefined Instruction
_svc_handler:
    b svc_handler              // 0x08: Supervisor Call (SVC)
_prefetch_abort_handler:
    b prefetch_abort_handler   // 0x0C: Prefetch Abort
_data_abort_handler:
    b data_abort_handler       // 0x10: Data Abort
_reserved_handler:
    b reserved_handler         // 0x14: Reserved
_irq_handler: 
    b irq_handler              // 0x18: IRQ
_fiq_handler:
    b fiq_handler              // 0x1C: FIQ

// 以下是各个异常的处理函数（你可以定义为具体功能）

reset_handler:
    RECORD_IRQ 0*4
    b __alltraps

undefined_handler:
    RECORD_IRQ 1*4
    b __alltraps

svc_handler:
    add     sp, sp, #-12
    RECORD_IRQ 2*4
    b __alltraps

prefetch_abort_handler:
    RECORD_IRQ 3*4
    b __alltraps

data_abort_handler:
    RECORD_IRQ 4*4
    b __alltraps

reserved_handler:
    RECORD_IRQ 5*4
    b __alltraps

irq_handler:
    RECORD_IRQ 6*4
    b __alltraps

fiq_handler:
    RECORD_IRQ 7*4
    b __alltraps


// Calling convention to __alltraps_svc:
// no register can be corrupted. (maybe svc call)
// sp cannot be corrupted. (maybe svc call)

__alltraps: 
    // addr = ffffe2c4.
    
    CMP     r7, #400          // 比较 r7 和 400
    BLE     __os_delegate       // 如果 r7 小于等于 400，跳转

    CMP     r8, #2*4          // 比较 r8 和 2*4 (svc)
    BNE     __os_delegate       // 如果 r8 不等于 svc，跳转

// Now it is extended syscall.
// 
__extsyscall:
    RESTORE_IRQ
    SAVE_CTX_TO_ADDR sp

    mov     r7, #0
    mov     r0, sp
    ldr     r8, =0xfffff000

    blx      r8

    LOAD_CTX_FROM_ADDR sp

    movs   pc, lr

// Now it is os delegate trap.
// 
__os_delegate:
    CONFIG_DATA r0
    LDR     r0, [r0, #1*4]
    # kernel vector table

    add     r0, r0, r8
    # add the shift

    ldr	    r7, [sp, #4]
    ldr	    r8, [sp, #8] 

    bx      r0
    // OS return the trap by itself.
    // No return back to trap gate any more.

    // Note: OS delegate format:
    // r0 is CORRUPTED, but r0 is stored in sp.
    // the first instruction in OS trap handler should be: ldr r0, [sp]

.ltorg

.align 12