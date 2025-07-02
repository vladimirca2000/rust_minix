#![no_std]
#![no_main]
#![cfg_attr(test, allow(unused_imports))]

// Inclui informações de build geradas pelo build.rs
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

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
    println!("Version: {}", KERNEL_VERSION);
    println!("Target: {} ({})", TARGET_ARCH, TARGET_CPU);
    println!("Build: {}", BUILD_TIME);
    println!("BSS cleared: {} bytes", bss_size);
    
    let stack_start = unsafe { &__stack_start as *const u8 as usize };
    println!("Stack start: 0x{:x}", stack_start);
    
    // Inicializa subsistemas de memória (incluindo alocador)
    memory::init();
    
    println!("DEBUG: Memory initialization completed");
    
    // Inicializar multi-core antes de outras inicializações
    println!("Initializing multi-core system...");
    arch::multicore::init_multicore();
    
    println!("DEBUG: Multi-core initialization completed");
    
    // Aguardar um pouco para cores secundários inicializarem
    unsafe { delay(10_000_000) };
    
    // Mostrar status multi-core detalhado
    arch::multicore::print_cores_status();
    
    // Inicializa a arquitetura (interrupções, timer, etc.)
    arch::aarch64::init();
    
    // Inicializa sistema de exceções
    println!("Setting up exception handling...");
    arch::exceptions::init();
    
    // Habilita interrupções após tudo estar configurado
    println!("Enabling interrupts...");
    arch::exceptions::enable_interrupts();
    
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
    
    // Informações do sistema multi-core
    let online_cores = arch::multicore::get_online_cores();
    println!("Multi-core system ready: {} cores online", online_cores);
    
    for core_id in 0..4 {
        if arch::multicore::is_core_online(core_id) {
            println!("  Core {} is online", core_id);
        }
    }
    
    // Teste do sistema de interrupções
    println!("Testing interrupt system...");
    println!("Interrupt status: {}", arch::exceptions::interrupts_enabled());
    
    // Demonstra o sistema funcionando sem sleep
    println!("Timer tick count before: {}", drivers::timer::get_tick_count());
    println!("System is ready and running!");
    
    // Loop principal do kernel
    let mut counter = 0;
    let mut multicore_demo_phase = 0;
    
    loop {
        // Mostra informações do sistema a cada 1M iterações
        if counter % 1000000 == 0 {
            let time = arch::aarch64::get_time();
            let tick_count = drivers::timer::get_tick_count();
            
            println!("Counter: {}, System time: {} ms, Timer ticks: {}", 
                     counter / 1000000, time, tick_count);
            
            // Demonstração multi-core em fases
            match multicore_demo_phase {
                0 => {
                    println!("🔄 Demo Phase 0: Setting cores to compute mode");
                    arch::multicore::set_cores_compute_mode();
                },
                2 => {
                    println!("📊 Demo Phase 2: Checking workload balance");
                    arch::multicore::balance_core_workload();
                },
                4 => {
                    println!("😴 Demo Phase 4: Setting cores to idle mode");
                    arch::multicore::set_cores_idle_mode();
                },
                6 => {
                    println!("📈 Demo Phase 6: Final status check");
                    arch::multicore::print_cores_status();
                },
                _ => {
                    // Fases intermediárias - apenas monitorar
                    if multicore_demo_phase % 2 == 1 {
                        let stats = arch::multicore::get_core_workload_stats();
                        println!("📊 Core workloads: [{}] [{}] [{}] [{}]", 
                                stats[0], stats[1], stats[2], stats[3]);
                    }
                }
            }
            
            multicore_demo_phase += 1;
            
            // Atualizar display visual
            drivers::display::draw_kernel_status(time, tick_count, counter / 1000000);
            
            // Se tivemos algum tick, o timer está funcionando!
            if tick_count > 0 {
                println!("🎉 Timer interrupts are working! Tick count: {}", tick_count);
            }
        }
        
        counter += 1;
        
        // Permite que interrupções sejam processadas
        core::hint::spin_loop();
        
        // Para após alguns loops para demonstração
        if counter >= 8000000 {
            println!("Test completed after {} iterations", counter);
            
            // Shutdown dos cores secundários antes de finalizar
            println!("🛑 Shutting down secondary cores...");
            arch::multicore::shutdown_secondary_cores();
            
            drivers::display::draw_kernel_status(
                arch::aarch64::get_time(), 
                drivers::timer::get_tick_count(), 
                counter / 1000000
            );
            println!("Entering infinite loop...");
            loop {
                core::hint::spin_loop();
            }
        }
    }
}