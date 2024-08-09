use std::sync::{Arc, Mutex};

use super::{Serial, Timer, LCD};

pub struct IO {
    serial: Serial,
    pub lcd: Arc<Mutex<LCD>>,
    pub timer: Timer,
    interrupt_flag_register: u8,
}

impl IO {
    pub fn new(lcd: Arc<Mutex<LCD>>) -> Self {
        Self {
            serial: Serial::new(),
            lcd,
            timer: Timer::new(),
            interrupt_flag_register: 0,
        }
    }

    pub fn get_if_flag(&self) -> u8 {
        self.interrupt_flag_register
    }

    pub fn set_if_flag(&mut self, value: u8) {
        self.interrupt_flag_register = value;
    }
    pub fn io_read(&self, address: u16) -> u8 {
        if address == 0xFF01 || address == 0xFF02 {
            self.serial.io_read(address)
        } else if address >= 0xFF04 && address <= 0xFF07 {
            self.timer.timer_read(address)
        } else if address == 0xFF0F {
            self.get_if_flag()
        } else if address >= 0xFF40 && address <= 0xFF4B {
            self.lcd.lock().unwrap().lcd_read(address)
        } else {
            println!("UNSUPPORTED io_read({:#02X}) - IO", address);
            //unimplemented!();
            0
        }
    }
    pub fn io_write(&mut self, address: u16, value: u8) {
        if address == 0xFF01 || address == 0xFF02 {
            self.serial.io_write(address, value);
        } else if address >= 0xFF04 && address <= 0xFF07 {
            self.timer.timer_write(address, value);
        } else if address == 0xFF0F {
            self.set_if_flag(value);
        } else if address >= 0xFF40 && address <= 0xFF4B {
            self.lcd.lock().unwrap().lcd_write(address, value);
        } else {
            println!("UNSUPPORTED io_write({:#02X}, {:#02X})", address, value);
            //unimplemented!();
        }
    }
}
