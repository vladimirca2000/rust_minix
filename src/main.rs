#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

// Habilita a crate alloc quando o feature estiver ativo
#[cfg(feature = "alloc")]
extern crate alloc;

mod arch;
mod boot;
mod drivers;
mod macros;
mod memory;
mod panic;

// Símbolos definidos no linker script
extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
    static __stack_start: u8;
    static __stack_end: u8;
}

// Handler para erros de alocação
use core::alloc::Layout;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Allocation error")
}

// Função de atraso para garantir estabilidade do console
unsafe fn delay(cycles: usize) {
    for _ in 0..cycles {
        core::arch::asm!("nop", options(nomem, nostack));
    }
}

// Ponto de entrada principal em Rust
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Limpeza da seção BSS
    let bss_size = unsafe {
        (&__bss_end as *const u8 as usize) - (&__bss_start as *const u8 as usize)
    };
    unsafe {
        memory::memzero(&__bss_start as *const u8 as *mut u8, bss_size);
    }

    // Inicializa drivers (UART primeiro para habilitar prints)
    drivers::init();
    
    // Aguarda estabilização do console
    unsafe {
        println!("Waiting for console to stabilize...");
        delay(50_000_000); // Ajuste conforme a velocidade do seu CPU
    }
    
    println!("=== Rust MINIX (ARM64) ===");
    println!("BSS cleared: {} bytes", bss_size);
    
    let stack_start = unsafe { &__stack_start as *const u8 as usize };
    println!("Stack start: 0x{:x}", stack_start);
    
    // Inicializa subsistemas de memória (incluindo alocador)
    memory::init();
    
    // Inicializa a arquitetura (interrupções, timer, etc.)
    arch::aarch64::init();
    
    // Teste de alocação dinâmica (só funciona com o feature 'alloc')
    #[cfg(feature = "alloc")]
    {
        use alloc::boxed::Box;
        let test_value = Box::new(42);
        println!("Allocated value: {}", *test_value);
        
        use alloc::vec::Vec;
        let mut numbers = Vec::new();
        numbers.push(10);
        numbers.push(20);
        numbers.push(30);
        println!("Vector contents: {:?}", numbers);
    }
    
    println!("System ready for Phase 2");


    
    // Loop principal do kernel
    loop {
        //mostre a hora atual
        let time = arch::aarch64::get_time();
        // mostra hora no centro da tela

        println!("Current time: {} ms", time);
    }
}