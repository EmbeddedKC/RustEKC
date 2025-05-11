ENTRY(_start)

BASE_ADDRESS = 0x60010000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    .text : {
        stext = .;
        *(.text._start)

        . = ALIGN(4K);
        strampoline = .;
        *(.text.trampoline)
        . = ALIGN(4K);

        ssignaltrampoline = .;
        *(.text.signaltrampoline)
        . = ALIGN(4K);

        snktrampoline = .;
        *(.text.nktrampoline)
        . = ALIGN(4K);

        *(.text .text.*)
        . = ALIGN(4K);
        
        etext = .;
    }
   
    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4K);
        erodata = .;
    }
    
    .data : {
        . = ALIGN(16K);
        sdata = .;
        *(.data.boot_page_table)
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4K);
        edata = .;
    }

    .bss : {
        sbss = .;
        boot_stack = .;
        *(.bss.stack)
        boot_stack_top = .;
        . = ALIGN(4K);
        sbss_no_stack = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        
        . = ALIGN(4K);
        sproxy = .;
        . = .+4K;
        eproxy = .;

        ebss = .;

    }

    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame) *(.eh_frame_hdr)
    }
}
