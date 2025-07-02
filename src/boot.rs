use core::arch::global_asm;

global_asm!(
    r#"
    .section ".text.boot"
    .global _start
    .global secondary_core_entry

    _start:
        // Obter ID do core
        mrs x0, mpidr_el1
        and x0, x0, #0xFF
        
        // Se não for core 0, hibernar até ser acordado
        cbz x0, primary_core_init
        
    secondary_wait:
        // Cores secundários aguardam por mailbox
        mov x1, #0x40000000    // Base do mailbox
        add x1, x1, #0x80      // Offset do mailbox
        lsl x2, x0, #4         // Core ID * 16
        add x1, x1, x2         // Endereço do mailbox para este core
        add x1, x1, #0x0C      // Offset do entry point
        
    secondary_check:
        ldr x2, [x1]           // Ler entry point do mailbox
        cbz x2, secondary_sleep // Se zero, continuar dormindo
        
        // Entry point válido encontrado, configurar stack temporário
        ldr x3, =0x90000       // Base stack secundário
        lsl x4, x0, #16        // Core ID * 64KB
        add x3, x3, x4         // Stack base para este core
        add x3, x3, #0x10000   // Topo do stack
        mov sp, x3
        
        // Limpar mailbox
        str xzr, [x1]
        
        // Pular para entry point
        br x2
        
    secondary_sleep:
        wfe                    // Wait for event
        b secondary_check
        
    primary_core_init:
        // Core 0: Configuração da stack
        ldr x0, =__stack_end
        mov sp, x0

        // Limpeza da seção .bss
        ldr x0, =__bss_start
        ldr x1, =__bss_end
        sub x1, x1, x0
        bl memzero

        // Chama o Rust main
        bl rust_main
        
    halt:
        wfe
        b halt
    
    // Entry point para cores secundários (chamado do Rust)
    secondary_core_entry:
        // Stack já foi configurado pelo código Rust
        // Pular para função Rust de cores secundários
        bl secondary_core_entry_rust
        
    secondary_halt:
        wfe
        b secondary_halt
    "#
);