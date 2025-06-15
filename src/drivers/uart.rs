use core::fmt;
use spin::Mutex;

const UART_BASE: usize = 0x3F20_1000;

#[repr(C, align(4))]
struct UartRegisters {
    dr: u32,
    _reserved0: [u32; 5],
    fr: u32,
    _reserved1: [u32; 1],
    ilpr: u32,
    ibrd: u32,
    fbrd: u32,
    lcr_h: u32,
    cr: u32,
    ifls: u32,
    imsc: u32,
    ris: u32,
    mis: u32,
    icr: u32,
    dmacr: u32,
}

pub struct Uart;

impl Uart {
    pub fn init(&self) {
        let regs = self.registers() as *mut u32;
        
        // Desabilita UART
        unsafe { regs.add(12).write_volatile(0) }; // CR offset
        
        // Configura baud rate (115200)
        let baud_rate_div = (48_000_000 / (16 * 115200)) as u32;
        unsafe {
            regs.add(9).write_volatile(baud_rate_div);  // IBRD
            regs.add(10).write_volatile(0);             // FBRD
        }
        
        // 8 bits, sem paridade, FIFO habilitado
        unsafe { regs.add(11).write_volatile(0b11 << 5 | 1 << 4) }; // LCR_H
        
        // Habilita UART, TX/RX
        unsafe { regs.add(12).write_volatile(0b1 << 0 | 0b1 << 8 | 0b1 << 9) }; // CR
    }
    
    fn registers(&self) -> *mut UartRegisters {
        UART_BASE as *mut UartRegisters
    }
    
    pub fn write_byte(&self, byte: u8) {
        let regs = self.registers() as *mut u32;
        
        // Espera até FIFO TX estar vazio
        while unsafe { regs.add(6).read_volatile() } & (1 << 5) != 0 {}
        
        unsafe { regs.write_volatile(byte as u32) }; // DR
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

// Usando Mutex para mutabilidade segura
pub static UART: Mutex<Uart> = Mutex::new(Uart);

pub fn init() {
    UART.lock().init();
}