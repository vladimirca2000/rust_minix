use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use spin::Mutex;

// Estado do alocador
struct AllocState {
    next: usize,
    heap_start: usize,
    heap_size: usize,
}

pub struct SimpleAllocator {
    state: Mutex<AllocState>,
}

impl SimpleAllocator {
    pub const fn new() -> Self {
        SimpleAllocator {
            state: Mutex::new(AllocState {
                next: 0,
                heap_start: 0,
                heap_size: 0,
            }),
        }
    }
    
    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        let mut state = self.state.lock();
        state.next = heap_start;
        state.heap_start = heap_start;
        state.heap_size = heap_size;
    }
}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut state = self.state.lock();
        let alloc_start = (state.next + layout.align() - 1) & !(layout.align() - 1);
        let alloc_end = alloc_start + layout.size();
        
        if alloc_end > state.heap_start + state.heap_size {
            null_mut()
        } else {
            state.next = alloc_end;
            alloc_start as *mut u8
        }
    }
    
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Implementação simplificada - não libera memória
    }
}

// Alocador global
#[global_allocator]
pub static ALLOCATOR: SimpleAllocator = SimpleAllocator::new();