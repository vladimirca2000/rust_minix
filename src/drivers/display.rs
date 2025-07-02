use crate::println;

// Simple display driver for kernel status output
// This provides a simulated visual display in QEMU
pub struct Display {
    line_count: usize,
}

static mut DISPLAY: Display = Display {
    line_count: 0,
};

#[allow(static_mut_refs)]
impl Display {
    pub fn write_line(&mut self, text: &str) {
        self.line_count += 1;
        println!("[DISPLAY LINE {}] {}", self.line_count, text);
    }
    
    pub fn clear(&mut self) {
        self.line_count = 0;
        println!("[DISPLAY] Screen cleared");
    }
}

#[allow(static_mut_refs)]
pub fn init() {
    println!("Display driver initialized (simulated mode)");
    unsafe {
        DISPLAY.clear();
        DISPLAY.write_line("=== Rust MINIX Display ===");
        DISPLAY.write_line("ARM64 Kernel for Raspberry Pi 3B+");
    }
}

#[allow(static_mut_refs)]
pub fn draw_kernel_status(time_ms: u64, tick_count: u64, counter: u64) {
    unsafe {
        DISPLAY.write_line("System Status Update:");
        DISPLAY.write_line("  System running normally");
        
        if tick_count > 0 {
            DISPLAY.write_line("🎉 INTERRUPTS WORKING!");
        } else {
            DISPLAY.write_line("⏳ Waiting for interrupts...");
        }
        
        // Use println! for detailed info since format! needs alloc
        println!("  System Time: {} ms", time_ms);
        println!("  Timer Ticks: {}", tick_count);
        println!("  Loop Counter: {}", counter);
        
        DISPLAY.write_line("---");
    }
}
