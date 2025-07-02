use core::arch::{asm, global_asm};
use crate::println;

// Vector table for exception handling
global_asm!(include_str!("exceptions.S"));

// Exception types
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ExceptionType {
    SynchronousEL1,
    IrqEL1,
    FiqEL1,
    SErrorEL1,
    SynchronousEL0,
    IrqEL0,
    FiqEL0,
    SErrorEL0,
}

// Exception context structure
#[repr(C)]
#[derive(Debug)]
pub struct ExceptionContext {
    pub gpr: [u64; 31],  // General purpose registers x0-x30
    pub sp_el0: u64,
    pub elr_el1: u64,    // Exception Link Register
    pub spsr_el1: u64,   // Saved Program Status Register
}

impl ExceptionContext {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ExceptionContext {
            gpr: [0; 31],
            sp_el0: 0,
            elr_el1: 0,
            spsr_el1: 0,
        }
    }
}

// Exception handlers called from assembly
#[no_mangle]
extern "C" fn sync_exception_el1(ctx: &mut ExceptionContext) {
    println!("Synchronous exception at EL1");
    println!("ELR_EL1: 0x{:016x}", ctx.elr_el1);
    println!("SPSR_EL1: 0x{:016x}", ctx.spsr_el1);
    
    // Read ESR_EL1 to get exception information
    let esr: u64;
    unsafe { asm!("mrs {}, esr_el1", out(reg) esr) };
    
    let exception_class = (esr >> 26) & 0x3F;
    let _instruction_length = (esr >> 25) & 1;
    
    println!("ESR_EL1: 0x{:016x}", esr);
    println!("Exception Class: 0x{:02x}", exception_class);
    
    match exception_class {
        0x15 => println!("SVC instruction execution"),
        0x21 => println!("Instruction abort from lower EL"),
        0x25 => println!("Data abort from lower EL"),
        _ => println!("Unknown exception class"),
    }
    
    // For now, halt on any synchronous exception
    panic!("Unhandled synchronous exception");
}

#[no_mangle]
extern "C" fn irq_exception_el1(_ctx: &mut ExceptionContext) {
    println!("IRQ exception at EL1");
    
    // Handle interrupt
    crate::drivers::gic::handle_irq();
    
    // IRQ handling is done, return normally
}

#[no_mangle]
extern "C" fn fiq_exception_el1(ctx: &mut ExceptionContext) {
    println!("FIQ exception at EL1");
    println!("ELR_EL1: 0x{:016x}", ctx.elr_el1);
    
    // For now, just return
    println!("FIQ handled, returning");
}

#[no_mangle]
extern "C" fn serror_exception_el1(ctx: &mut ExceptionContext) {
    println!("SError exception at EL1");
    println!("ELR_EL1: 0x{:016x}", ctx.elr_el1);
    println!("SPSR_EL1: 0x{:016x}", ctx.spsr_el1);
    
    panic!("Unhandled SError exception");
}

#[no_mangle]
extern "C" fn sync_exception_el0(ctx: &mut ExceptionContext) {
    println!("Synchronous exception from EL0");
    println!("ELR_EL1: 0x{:016x}", ctx.elr_el1);
    
    // Read ESR_EL1 to get exception information
    let esr: u64;
    unsafe { asm!("mrs {}, esr_el1", out(reg) esr) };
    
    let exception_class = (esr >> 26) & 0x3F;
    println!("Exception Class: 0x{:02x}", exception_class);
    
    match exception_class {
        0x15 => {
            // SVC (System Call)
            println!("System call from EL0");
            handle_syscall(ctx);
        }
        _ => {
            println!("Unhandled EL0 exception");
            panic!("Unhandled EL0 exception");
        }
    }
}

#[no_mangle]
extern "C" fn irq_exception_el0(_ctx: &mut ExceptionContext) {
    println!("IRQ exception from EL0");
    
    // Handle interrupt
    crate::drivers::gic::handle_irq();
}

#[no_mangle]
extern "C" fn fiq_exception_el0(_ctx: &mut ExceptionContext) {
    println!("FIQ exception from EL0");
    println!("FIQ handled, returning");
}

#[no_mangle]
extern "C" fn serror_exception_el0(_ctx: &mut ExceptionContext) {
    println!("SError exception from EL0");
    panic!("Unhandled SError from EL0");
}

fn handle_syscall(ctx: &mut ExceptionContext) {
    // System call number is in x8
    let syscall_num = ctx.gpr[8];
    
    println!("System call #{}", syscall_num);
    
    // For now, just return success (0) in x0
    ctx.gpr[0] = 0;
}

// Initialize exception handling
pub fn init() {
    println!("Setting up exception vector table...");
    
    unsafe {
        // Set VBAR_EL1 to point to our exception vector table
        asm!("adr x0, exception_vector_table");
        asm!("msr vbar_el1, x0");
    }
    
    println!("Exception vector table installed");
}

// Enable interrupts
pub fn enable_interrupts() {
    unsafe {
        asm!("msr daifclr, #2"); // Clear IRQ mask
    }
    println!("IRQ interrupts enabled");
}

// Disable interrupts
#[allow(dead_code)]
pub fn disable_interrupts() {
    unsafe {
        asm!("msr daifset, #2"); // Set IRQ mask
    }
    println!("IRQ interrupts disabled");
}

// Check if interrupts are enabled
pub fn interrupts_enabled() -> bool {
    let daif: u64;
    unsafe { asm!("mrs {}, daif", out(reg) daif) };
    (daif & 0x80) == 0 // IRQ mask bit
}
