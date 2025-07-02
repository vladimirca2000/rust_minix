use crate::println;
use core::arch::asm;

pub fn init() {
    // Desabilita todas interrupções
    unsafe {
        asm!("msr daifset, #0xf");
    }
    println!("Interrupts disabled");
    
    // Obtém nível de exceção atual
    let el = current_el();
    println!("Running at EL{}", el);
    
    // Obtém ID do core
    let core_id = get_core_id();
    println!("Running on core {}", core_id);
    println!("DEBUG: Core ID = {}, comparing with 0", core_id);
}

fn current_el() -> u32 {
    let el: u64;
    unsafe { asm!("mrs {}, CurrentEL", out(reg) el) };
    ((el >> 2) & 0x3) as u32
}

fn get_core_id() -> u8 {
    let id: u64;
    unsafe { asm!("mrs {}, mpidr_el1", out(reg) id) };
    (id & 0xFF) as u8
}

pub fn get_time() -> u64 {
        // On AArch64, you can use the system counter
        let counter: u64;
        unsafe {
            core::arch::asm!("mrs {}, cntpct_el0", out(reg) counter);
        }
        
        // Convert counter ticks to milliseconds
        // You need to know your counter frequency to do this correctly
        // This example assumes a 1MHz counter
        counter / 1000
    }