.section .text
////////////////////////////
// Assuming that OS has initialized all the stack pointer.
////////////////////////////


// FIXME:
// This macro would save all the context from user, but:
// the trap handler cannot see the correct value of lr and r10 of user!
// correct: the lr shift 

.macro IRQ_TO_SVC c ty
	stmia	sp, {r9, r10}		@ save r10, r9
	mrs	r10, spsr
	str	r10, [sp, #8]		@ save spsr

    // correct the lr return.
    add lr, lr, #4
    str	lr, [sp, #12]

    // a fake SVC state into spsr.
	mrs	r10, cpsr
    bic r10, #0x1F
	eor	r10, r10, #0x13 
	msr	spsr_cxsf, r10

    // keep r9 the previous IRQ state
    ldr	r9, [sp, #8]

    // keep r10 the current IRQ state
    mrs	r10, cpsr

    // "return" to SVC state
    // lr cannot be changed in the special state.
    movs pc, pc

    // disable irq fiq
    cpsid i
    cpsid f

    // save the "current" IRQ state into spsr.
    msr	spsr_cxsf, r10

    // set the SVC lr to irq return. 
    // Here must use adr instead of ldr, because PC-relative are required in trampoline
    adr lr, irq_exit
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
    IRQ_TO_SVC 4
    add sp, sp, #-20*4
    SAVE_CTX_TO_ADDR sp
    mov r8, #0
    b __alltraps

undefined_handler:
    IRQ_TO_SVC 4
    add sp, sp, #-20*4
    SAVE_CTX_TO_ADDR sp
    mov r8, #1
    b __alltraps

svc_handler:
    add sp, sp, #-20*4
    SAVE_CTX_TO_ADDR sp

    // let r9 be previous IRQ state.
    mrs	r9, spsr

    mov r8, #2
    b __alltraps


prefetch_abort_handler:
    IRQ_TO_SVC 0
    add sp, sp, #-20*4
    SAVE_CTX_TO_ADDR sp
    mov r8, #3
    b __alltraps

data_abort_handler:
    IRQ_TO_SVC 0
    add sp, sp, #-20*4
    SAVE_CTX_TO_ADDR sp
    mov r8, #4
    b __alltraps

reserved_handler:
    IRQ_TO_SVC 4
    add sp, sp, #-20*4
    SAVE_CTX_TO_ADDR sp
    mov r8, #5
    b __alltraps

irq_handler:
    IRQ_TO_SVC 4
    add sp, sp, #-20*4
    SAVE_CTX_TO_ADDR sp
    mov r8, #6
    b __alltraps

fiq_handler:
    IRQ_TO_SVC 4
    add sp, sp, #-20*4
    SAVE_CTX_TO_ADDR sp
    mov r8, #7
    b __alltraps


// Calling convention to __alltraps:
// r8 - the interrupt type.
// r9 - the caller CPU state register (spsr).
// __alltraps wont protect any register context.

__alltraps: 
    // addr = ffffe2c4.
    
    CMP     r8, #2            // 比较 r8 和 2
    BNE     os_delegate       // 如果 r8 不等于 2，跳转

    CMP     r7, #400          // 比较 r7 和 400
    BLE     os_delegate       // 如果 r7 小于等于 400，跳转

    // Now it is extended syscall.
    
    mov     r7, #0
    mov     r0, sp
    ldr     r8, =0xfffff000

    blx      r8

    b trap_exit

os_delegate:
    mov     r1, r8
    CONFIG_DATA r8
    
    # get the previous mode
    AND     r2, r9, #0x1F   

    MOV     r0, sp
    LDR     r9, [r8, #1*4]
    # kernel trap handler

    blx      r9

    # A little difference:
    # The func param is 0=trapctx, 1=intr type, 2=previous cpu state

    b trap_exit

trap_exit:  @ addr = 0xffffe304
    LOAD_CTX_FROM_ADDR sp
    add sp, sp, #20*4
    movs   pc, lr

irq_exit:   @ addr = 0xffffe320
	ldr	r10, [sp, #8]		@ load user spsr back
    msr	spsr, r10        @ load user spsr back

    ldmia	sp, {r10, r11}   @ load user r10 and r11 backc
    ldr	lr, [sp, #12]		@ load lr back
    movs    pc, lr          @ return to user.  0xffffe32c


.ltorg

.align 12