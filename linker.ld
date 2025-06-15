ENTRY(_start)

MEMORY {
    RAM (rwx) : ORIGIN = 0x80000, LENGTH = 128M
}

SECTIONS {
    . = ORIGIN(RAM);
    
    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }
    
    .rodata : ALIGN(8) {
        *(.rodata .rodata.*)
    }
    
    .data : ALIGN(8) {
        *(.data .data.*)
    }
    
    .bss : ALIGN(8) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
    }
    
    . = ALIGN(4096);
    __stack_start = .;
    . += 0x10000;
    __stack_end = .;
    
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
    }
    
    __kernel_end = .;
}