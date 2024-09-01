use core::fmt;
use std::{
    fmt::{Debug, Display},
    sync::{Arc, Mutex},
};

use rusty_gb::bus::Bus;

#[derive(Debug)]
pub struct TestBus {
    data: Vec<u8>,
}

impl Display for TestBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_str: String = String::new();
        for byte in self.data.iter() {
            output_str = format!("{}, {:02X} ", output_str, byte);
        }
        write!(f, "{} ", output_str)
    }
}

impl TestBus {
    pub fn create(data: Vec<u8>) -> Arc<Mutex<TestBus>> {
        Arc::new(Mutex::new(TestBus { data }))
    }
}

impl Bus for TestBus {
    fn bus_read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn bus_write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    fn dbg_update(&mut self) {}

    fn dbg_print(&self) {}

    fn timer_tick(&mut self) -> Option<rusty_gb::cpu::types::InterruptType> {
        None
    }

    fn dma_tick(&mut self) -> bool {
        return false;
    }
}
