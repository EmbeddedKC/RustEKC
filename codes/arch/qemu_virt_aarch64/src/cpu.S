.macro return
    br x30
.endm

.globl cpu_time
cpu_time:
    mrs x0, cntvct_el0
    return
    

.globl cpu_id
cpu_id:
    mrs x0, mpidr_el1
    return


.globl cpu_freq
cpu_freq:
    mrs x0, cntfrq_el0
    return


.globl cpu_sync
cpu_sync:
    dsb ish
    isb
    return
