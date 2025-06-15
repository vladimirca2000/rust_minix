pub mod mmu;
pub mod allocator;

use crate::println;

// Definir região do heap
const HEAP_START: usize = 0x40000000; // Endereço arbitrário
const HEAP_SIZE: usize = 0x10000; // 64KB

#[no_mangle]
pub extern "C" fn memzero(dest: *mut u8, size: usize) {
    for i in 0..size {
        unsafe { *dest.add(i) = 0; }
    }
}

pub fn init() {
    println!("Initializing memory subsystems...");
    
    // Inicializar alocador
    unsafe {
        allocator::ALLOCATOR.init(HEAP_START, HEAP_SIZE);
    }
    println!("Heap initialized at 0x{:x} (size: {} bytes)", HEAP_START, HEAP_SIZE);
    
    mmu::init();
}