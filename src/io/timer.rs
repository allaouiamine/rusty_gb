use crate::cpu::context::InterruptType;

pub struct Timer {
    div: u16, // 0xFF04
    tima: u8, // 0xFF05
    tma: u8,  // 0xFF06
    tac: u8,  // 0xFF07
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xABCC,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn timer_tick(&mut self) -> Option<InterruptType> {
        let previous_div = self.div;
        let timer_enabled = self.tac & 0b100 == 0b100;

        self.div += 1;

        if !timer_enabled {
            return None;
        }

        let div_value = match self.tac & 0b11 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unimplemented!(),
        };

        if previous_div & div_value == div_value && self.div & div_value == 0 {
            match self.tima.checked_add(1) {
                Some(v) => {
                    self.tima = v;
                    None
                }
                None => {
                    self.tima = self.tma;
                    Some(InterruptType::TIMER)
                }
            }
        } else {
            None
        }
    }
    pub fn timer_write(&mut self, address: u16, value: u8) {
        if address == 0xFF04 {
            self.div = 0;
        } else if address == 0xFF05 {
            self.tima = value;
        } else if address == 0xFF06 {
            self.tma = value;
        } else if address == 0xFF07 {
            self.tac = value;
        } else {
            unimplemented!();
        }
    }
    pub fn timer_read(&self, address: u16) -> u8 {
        if address == 0xFF04 {
            (self.div >> 8) as u8
        } else if address == 0xFF05 {
            self.tima
        } else if address == 0xFF06 {
            self.tma
        } else if address == 0xFF07 {
            self.tac
        } else {
            unimplemented!();
        }
    }
}
