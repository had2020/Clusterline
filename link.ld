SECTIONS
{
    /*. = 0x100000; /* Kernel_start */

    .text   : { *(.text) }
    .rodata : { *(.rodata) }
    .data   : { *(.data) }
    .bss    : { *(.bss) }

    /*. = ALIGN(4K);*/
    
    _kernel_end = .; 
}
