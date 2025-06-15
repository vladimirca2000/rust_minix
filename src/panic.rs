use core::panic::PanicInfo;
use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("--- KERNEL PANIC ---");
    
    // Informações de localização
    if let Some(location) = info.location() {
        println!(
            "File: {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    }
    
    // Mensagem de panic
    println!("Reason: {}", info.message());
    
    println!("-------------------");
    
    loop {}
}