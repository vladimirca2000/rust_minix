use core::arch::global_asm;

global_asm!(
    r#"
    .section ".text.boot"
    .global _start

    _start:
        // Só o core 0 continua, outros hibernam
        mrs x0, mpidr_el1
        and x0, x0, #0xFF
        cbz x0, 2f
    1:  wfe
        b 1b

    2:  // Configuração da stack
        ldr x0, =__stack_end
        mov sp, x0

        // Limpeza da seção .bss
        ldr x0, =__bss_start
        ldr x1, =__bss_end
        sub x1, x1, x0
        bl memzero

        // Chama o Rust
        bl rust_main
        b 1b
    "#
);