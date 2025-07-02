pub mod uart;
pub mod gic;
pub mod timer;
pub mod display;

use crate::println;

pub fn init() {
    uart::init();
    gic::init();
    timer::init();
    display::init();
    println!("Drivers initialized");
}