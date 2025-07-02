/*
 * Linker Script para Rust MINIX ARM64
 * Target: Raspberry Pi 3B+ (Cortex-A53)
 * Memória: RAM inicia em 0x80000 (512KB offset)
 */

ENTRY(_start)

MEMORY {
    /* Raspberry Pi 3B+ tem 1GB RAM, kernel carregado em 0x80000 */
    RAM (rwx) : ORIGIN = 0x80000, LENGTH = 128M
}

SECTIONS {
    . = ORIGIN(RAM);
    
    /* Seção de código executável */
    .text : ALIGN(4096) {
        __text_start = .;
        KEEP(*(.text.boot))  /* Boot code deve vir primeiro */
        *(.text .text.*)
        __text_end = .;
    } > RAM
    
    /* Dados somente leitura */
    .rodata : ALIGN(8) {
        __rodata_start = .;
        *(.rodata .rodata.*)
        *(.strings)
        __rodata_end = .;
    } > RAM
    
    /* Dados inicializados */
    .data : ALIGN(8) {
        __data_start = .;
        *(.data .data.*)
        __data_end = .;
    } > RAM
    
    /* Dados não inicializados (zeroed) */
    .bss : ALIGN(8) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
    } > RAM
    
    /* Stack do kernel primário (64KB) */
    . = ALIGN(4096);
    __stack_start = .;
    . += 0x10000;  /* 64KB stack */
    __stack_end = .;
    
    /* Stacks para cores secundários (64KB cada) */
    . = ALIGN(4096);
    __secondary_stacks_start = .;
    . += 0x30000;  /* 3 x 64KB para cores 1, 2, 3 */
    __secondary_stacks_end = .;
    
    /* Heap start (para alocador dinâmico) */
    . = ALIGN(4096);
    __heap_start = .;
    
    /* Informações de debug e metadata (descartadas no binário final) */
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.note*)
        *(.debug*)
    }
    
    /* Símbolo para fim do kernel */
    __kernel_end = .;
    
    /* Tamanho total do kernel */
    __kernel_size = __kernel_end - ORIGIN(RAM);
}