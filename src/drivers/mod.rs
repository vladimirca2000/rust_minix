pub mod uart;

use crate::println;

pub fn init() {
    uart::init();
    println!("Drivers initialized");
}