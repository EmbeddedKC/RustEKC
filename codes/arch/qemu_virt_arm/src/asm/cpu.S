.macro return
    bx LR
.endm

.globl cpu_time
cpu_time:
    MRC p15, 0, r0, c14, c0, 2
    return
    
.globl cpu_id
cpu_id:
    MRC p15, 0, r0, c0, c0, 5
    return

.globl cpu_freq
cpu_freq:
    MRC p15, 0, r0, c14, c0, 0
    return

.globl cpu_sync
cpu_sync:
    dsb ish
    isb
    return

.globl write_root_pt
write_root_pt:
    MCR p15, 0, r0, c2, c0, 0
    return

.globl enable_intr
enable_intr:
    CPSIE I
    CPSIE F
    return

.globl disable_intr
disable_intr:
    CPSID I
    CPSID F
    return

