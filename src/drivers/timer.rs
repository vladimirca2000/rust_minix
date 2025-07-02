use crate::println;
use crate::drivers::gic;

static mut TICK_COUNT: u64 = 0;

// Timer interrupt handler
#[allow(static_mut_refs)]
fn timer_interrupt_handler() {
    unsafe {
        TICK_COUNT += 1;
        
        // Print tick every 100 ticks (approximately every second if 10ms ticks)
        if TICK_COUNT % 100 == 0 {
            println!("Timer tick: {}", TICK_COUNT);
        }
    }
}

pub fn init() {
    println!("Initializing timer system...");
    
    // Register our timer handler
    gic::register_timer_handler(timer_interrupt_handler);
    
    // Setup timer for 10ms intervals (10000 microseconds)
    gic::setup_timer(10000);
    
    println!("Timer system initialized");
}

pub fn get_tick_count() -> u64 {
    unsafe { TICK_COUNT }
}

// Sleep for approximately the given number of ticks
#[allow(dead_code)]
pub fn sleep_ticks(ticks: u64) {
    let start_tick = get_tick_count();
    while get_tick_count() - start_tick < ticks {
        // Wait for timer interrupts
        core::hint::spin_loop();
    }
}

// Sleep for approximately the given number of milliseconds
#[allow(dead_code)]
pub fn sleep_ms(ms: u64) {
    // Each tick is 10ms, so divide by 10
    let ticks = ms / 10;
    if ticks > 0 {
        sleep_ticks(ticks);
    }
}
