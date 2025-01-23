OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x40200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        . = ALIGN(4K);

        strampoline = .;
        *(.text.trampoline);
        . = ALIGN(4K);

        snktrampoline = .;
        *(.text.nktrampoline);
        . = ALIGN(4K);

        ssignaltrampoline = .;
        *(.text.signaltrampoline);
        . = ALIGN(4K);
        
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
    }

    . = ALIGN(4K);
    edata = .;
    
    sbss = .;
    .bss : {
        *(.bss.stack)
        sbss_no_stack = .;
        *(.bss .bss.*)

        . = ALIGN(4K);
        sproxy = .;
        . = .+4K;
        eproxy = .;
    }

    . = ALIGN(4K);
    ebss = .;
    
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
