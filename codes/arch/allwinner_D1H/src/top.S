.altmacro
.macro PROXYCONTEXT n
    li \n, -0x3000
.endm
.altmacro
.macro MMKAPI_TABLE n
    li \n, -0x3000+0x800
.endm
.altmacro
.macro CONFIG_DATA n
    li \n, -0x3000+0x400
.endm

.include "src/nk_gate.S"
.include "src/trap/trap.S"
.include "src/trap/trap_signal.S"