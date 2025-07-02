use core::ptr::{read_volatile, write_volatile};
use crate::println;

// BCM2837 interrupt controller base addresses
const INTERRUPT_BASE: usize = 0x3F00B000;

// Local interrupt controller (for ARM timers)
const LOCAL_INTERRUPT_BASE: usize = 0x40000000;

// Interrupt controller registers
const IRQ_BASIC_PENDING: usize = INTERRUPT_BASE + 0x200;
const IRQ_PENDING_1: usize = INTERRUPT_BASE + 0x204;
const IRQ_PENDING_2: usize = INTERRUPT_BASE + 0x208;
#[allow(dead_code)]
const FIQ_CONTROL: usize = INTERRUPT_BASE + 0x20C;
const ENABLE_IRQS_1: usize = INTERRUPT_BASE + 0x210;
const ENABLE_IRQS_2: usize = INTERRUPT_BASE + 0x214;
#[allow(dead_code)]
const ENABLE_BASIC_IRQS: usize = INTERRUPT_BASE + 0x218;
const DISABLE_IRQS_1: usize = INTERRUPT_BASE + 0x21C;
const DISABLE_IRQS_2: usize = INTERRUPT_BASE + 0x220;
const DISABLE_BASIC_IRQS: usize = INTERRUPT_BASE + 0x224;

// Local interrupt controller registers
#[allow(dead_code)]
const LOCAL_CONTROL: usize = LOCAL_INTERRUPT_BASE + 0x00;
const LOCAL_IRQ_PENDING: usize = LOCAL_INTERRUPT_BASE + 0x60;
const LOCAL_IRQ_ENABLE: usize = LOCAL_INTERRUPT_BASE + 0x64;
const LOCAL_IRQ_DISABLE: usize = LOCAL_INTERRUPT_BASE + 0x68;

// Timer interrupt registers
const LOCAL_TIMER_IRQ_CONTROL: usize = LOCAL_INTERRUPT_BASE + 0x40;
const LOCAL_TIMER_IRQ_CLEAR: usize = LOCAL_INTERRUPT_BASE + 0x44;
const LOCAL_TIMER_IRQ_RELOAD: usize = LOCAL_INTERRUPT_BASE + 0x48;
#[allow(dead_code)]
const LOCAL_TIMER_IRQ_VALUE: usize = LOCAL_INTERRUPT_BASE + 0x4C;

// Interrupt numbers
pub const IRQ_UART: u32 = 57;
pub const IRQ_TIMER: u32 = 64;  // Local timer
pub const IRQ_MAILBOX: u32 = 65;

// Interrupt types
#[derive(Debug, Clone, Copy)]
pub enum InterruptType {
    Timer,
    Uart,
    Mailbox,
    Unknown(u32),
}

// Interrupt handler type
pub type InterruptHandler = fn();

// Static interrupt handlers
static mut TIMER_HANDLER: Option<InterruptHandler> = None;
static mut UART_HANDLER: Option<InterruptHandler> = None;

pub struct Gic {
    // GIC state can be stored here if needed
}

impl Gic {
    pub fn new() -> Self {
        Gic {}
    }

    pub fn init(&self) {
        println!("Initializing BCM2837 interrupt controller...");
        
        // Disable all interrupts initially
        unsafe {
            write_volatile(DISABLE_IRQS_1 as *mut u32, 0xFFFFFFFF);
            write_volatile(DISABLE_IRQS_2 as *mut u32, 0xFFFFFFFF);
            write_volatile(DISABLE_BASIC_IRQS as *mut u32, 0xFFFFFFFF);
            write_volatile(LOCAL_IRQ_DISABLE as *mut u32, 0xFFFFFFFF);
        }
        
        // Clear any pending interrupts
        self.clear_pending_interrupts();
        
        println!("Interrupt controller initialized");
    }

    pub fn enable_irq(&self, irq: u32) {
        match irq {
            0..=31 => {
                // IRQs 0-31 are in ENABLE_IRQS_1
                let bit = 1 << irq;
                unsafe {
                    write_volatile(ENABLE_IRQS_1 as *mut u32, bit);
                }
                println!("Enabled IRQ {} (bank 1)", irq);
            }
            32..=63 => {
                // IRQs 32-63 are in ENABLE_IRQS_2
                let bit = 1 << (irq - 32);
                unsafe {
                    write_volatile(ENABLE_IRQS_2 as *mut u32, bit);
                }
                println!("Enabled IRQ {} (bank 2)", irq);
            }
            64..=71 => {
                // Local interrupts (64-71)
                let bit = 1 << (irq - 64);
                unsafe {
                    write_volatile(LOCAL_IRQ_ENABLE as *mut u32, bit);
                }
                println!("Enabled local IRQ {}", irq);
            }
            _ => {
                println!("Invalid IRQ number: {}", irq);
            }
        }
    }

    #[allow(dead_code)]
    pub fn disable_irq(&self, irq: u32) {
        match irq {
            0..=31 => {
                let bit = 1 << irq;
                unsafe {
                    write_volatile(DISABLE_IRQS_1 as *mut u32, bit);
                }
            }
            32..=63 => {
                let bit = 1 << (irq - 32);
                unsafe {
                    write_volatile(DISABLE_IRQS_2 as *mut u32, bit);
                }
            }
            64..=71 => {
                let bit = 1 << (irq - 64);
                unsafe {
                    write_volatile(LOCAL_IRQ_DISABLE as *mut u32, bit);
                }
            }
            _ => {
                println!("Invalid IRQ number: {}", irq);
            }
        }
    }

    fn clear_pending_interrupts(&self) {
        // Clear any pending interrupts by reading the pending registers
        unsafe {
            let _ = read_volatile(IRQ_BASIC_PENDING as *const u32);
            let _ = read_volatile(IRQ_PENDING_1 as *const u32);
            let _ = read_volatile(IRQ_PENDING_2 as *const u32);
            let _ = read_volatile(LOCAL_IRQ_PENDING as *const u32);
        }
    }

    fn get_pending_irq(&self) -> Option<u32> {
        unsafe {
            // Check local interrupts first (higher priority)
            let local_pending = read_volatile(LOCAL_IRQ_PENDING as *const u32);
            if local_pending != 0 {
                for i in 0..8 {
                    if (local_pending & (1 << i)) != 0 {
                        return Some(64 + i);
                    }
                }
            }

            // Check basic pending register
            let basic_pending = read_volatile(IRQ_BASIC_PENDING as *const u32);
            if basic_pending != 0 {
                // Bits 0-7 in basic pending correspond to some high-priority IRQs
                for i in 0..8 {
                    if (basic_pending & (1 << i)) != 0 {
                        return Some(i);
                    }
                }
            }

            // Check IRQ pending registers
            let pending1 = read_volatile(IRQ_PENDING_1 as *const u32);
            if pending1 != 0 {
                for i in 0..32 {
                    if (pending1 & (1 << i)) != 0 {
                        return Some(i);
                    }
                }
            }

            let pending2 = read_volatile(IRQ_PENDING_2 as *const u32);
            if pending2 != 0 {
                for i in 0..32 {
                    if (pending2 & (1 << i)) != 0 {
                        return Some(32 + i);
                    }
                }
            }
        }

        None
    }

    pub fn handle_irq(&self) -> Option<InterruptType> {
        if let Some(irq_num) = self.get_pending_irq() {
            let interrupt_type = match irq_num {
                IRQ_TIMER => InterruptType::Timer,
                IRQ_UART => InterruptType::Uart,
                IRQ_MAILBOX => InterruptType::Mailbox,
                _ => InterruptType::Unknown(irq_num),
            };

            // Call the appropriate handler
            match interrupt_type {
                InterruptType::Timer => {
                    unsafe {
                        if let Some(handler) = TIMER_HANDLER {
                            handler();
                        }
                    }
                    // Clear timer interrupt
                    self.clear_timer_interrupt();
                }
                InterruptType::Uart => {
                    unsafe {
                        if let Some(handler) = UART_HANDLER {
                            handler();
                        }
                    }
                }
                InterruptType::Mailbox => {
                    println!("Mailbox interrupt");
                }
                InterruptType::Unknown(num) => {
                    println!("Unknown interrupt: {}", num);
                }
            }

            Some(interrupt_type)
        } else {
            None
        }
    }

    pub fn setup_timer(&self, interval_us: u32) {
        println!("Setting up local timer with interval {} us", interval_us);
        
        unsafe {
            // Disable timer first
            write_volatile(LOCAL_TIMER_IRQ_CONTROL as *mut u32, 0);
            
            // Set reload value (timer counts down)
            write_volatile(LOCAL_TIMER_IRQ_RELOAD as *mut u32, interval_us);
            
            // Enable timer with interrupt
            // Bit 28: timer enable, Bit 29: interrupt enable, Bit 30: reload
            write_volatile(LOCAL_TIMER_IRQ_CONTROL as *mut u32, 
                          (1 << 28) | (1 << 29) | (1 << 30));
        }
        
        // Enable timer interrupt
        self.enable_irq(IRQ_TIMER);
        
        println!("Local timer configured and enabled");
    }

    fn clear_timer_interrupt(&self) {
        unsafe {
            // Clear timer interrupt by writing to clear register
            write_volatile(LOCAL_TIMER_IRQ_CLEAR as *mut u32, 1);
        }
    }
}

// Global GIC instance
static mut GIC: Option<Gic> = None;

pub fn init() {
    let gic = Gic::new();
    gic.init();
    
    unsafe {
        GIC = Some(gic);
    }
}

#[allow(dead_code)]
#[allow(static_mut_refs)]
pub fn get_gic() -> &'static Gic {
    unsafe {
        GIC.as_ref().expect("GIC not initialized")
    }
}

#[allow(static_mut_refs)]
pub fn handle_irq() {
    if let Some(gic) = unsafe { GIC.as_ref() } {
        if let Some(_interrupt_type) = gic.handle_irq() {
            // Interrupt handled successfully
        } else {
            println!("Spurious interrupt");
        }
    }
}

// Register interrupt handlers
pub fn register_timer_handler(handler: InterruptHandler) {
    unsafe {
        TIMER_HANDLER = Some(handler);
    }
    println!("Timer interrupt handler registered");
}

#[allow(dead_code)]
pub fn register_uart_handler(handler: InterruptHandler) {
    unsafe {
        UART_HANDLER = Some(handler);
    }
    println!("UART interrupt handler registered");
}

// Setup timer with callback
#[allow(static_mut_refs)]
pub fn setup_timer(interval_us: u32) {
    if let Some(gic) = unsafe { GIC.as_ref() } {
        gic.setup_timer(interval_us);
    }
}

// Enable/disable specific interrupts
#[allow(dead_code)]
#[allow(static_mut_refs)]
pub fn enable_irq(irq: u32) {
    if let Some(gic) = unsafe { GIC.as_ref() } {
        gic.enable_irq(irq);
    }
}

#[allow(dead_code)]
#[allow(static_mut_refs)]
pub fn disable_irq(irq: u32) {
    if let Some(gic) = unsafe { GIC.as_ref() } {
        gic.disable_irq(irq);
    }
}
