.section .text


.align 12

.globl ssignaltrampoline;

ssignaltrampoline:
    # ecall sys_sigreturn
    mov     r7, #139
    svc 0


.align 12