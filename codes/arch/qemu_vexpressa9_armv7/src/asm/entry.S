.equ    MODE_USR,   0x10    // User mode
.equ    MODE_FIQ,   0x11    // FIQ mode
.equ    MODE_IRQ,   0x12    // IRQ mode
.equ    MODE_SVC,   0x13    // Supervisor mode (系统调用进入内核的模式)
.equ    MODE_ABT,   0x17    // Abort mode
.equ    MODE_UND,   0x1B    // Undefined instruction mode
.equ    MODE_SYS,   0x1F    // System mode (特权级的用户代码，常用于内核运行)

.section .text._start
.globl _start

_start:

    cpsid if

    # mem access controller.
    LDR     r8, =0xFFFFFFFF
    MCR     p15, 0, r8, c3, c0, 0

    mrs r8, cpsr

    bic r8, r8, #0x1F 
    orr r8, r8, #MODE_SVC
    msr     CPSR_c, r8
    ldr     sp, =svc_stack_top

    bic r8, r8, #0x1F
    orr r8, r8, #MODE_IRQ
    msr     CPSR_c, r8
    ldr     sp, =irq_stack_top

    bic r8, r8, #0x1F
    orr r8, r8, #MODE_ABT
    msr     CPSR_c, r8
    ldr     sp, =abt_stack_top

    bic r8, r8, #0x1F
    orr r8, r8, #MODE_UND
    msr     CPSR_c, r8
    ldr     sp, =und_stack_top

    bic r8, r8, #0x1F
    orr r8, r8, #MODE_FIQ
    msr     CPSR_c, r8
    ldr     sp, =fiq_stack_top

    bic r8, r8, #0x1F
    orr r8, r8, #MODE_SYS
    msr     CPSR_c, r8
    ldr     sp, =sys_stack_top

    //switch to SVC.
    bic r8, r8, #0x1F
    orr r8, r8, #MODE_SVC
    msr     CPSR_c, r8

    ldr     sp, =boot_stack_top

    STMFD SP!, {R0-R4}
    MOV     R0, sp
    add     sp, sp,#-4

    mov     r11, sp

    bl      estart


.section .bss.stack

.align 3

.equ STACK_SIZE, 0x1000   // 每种模式 4KB 栈空间

.global __stack_start

__stack_start:
    .space STACK_SIZE
    .global fiq_stack_top
fiq_stack_top:
    .space STACK_SIZE
    .global irq_stack_top
irq_stack_top:
    .space STACK_SIZE
    .global svc_stack_top
svc_stack_top:
    .space STACK_SIZE
    .global abt_stack_top
abt_stack_top:
    .space STACK_SIZE
    .global und_stack_top
und_stack_top:
    .space STACK_SIZE
    .global sys_stack_top
sys_stack_top:
    .space STACK_SIZE
