use crate::cpu::context::InterruptType;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Timer {
    div: u16, // 0xFF04 divider register
    tima: u8, // 0xFF05 Timer counter
    tma: u8,  // 0xFF06 Timer modulo
    tac: u8,  // 0xFF07 Timer control
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn timer_tick(&mut self) -> Option<InterruptType> {
        let prev_div = self.div;
        self.div += 1;

        let tac = self.tac & 0b111;
        let timer_enabled = tac >> 2 == 1;

        if !timer_enabled {
            return None;
        }

        let bit_shift_count: u8 = match self.tac & 0b11 {
            0b00 => 10,
            0b01 => 4,
            0b10 => 6,
            0b11 => 8,
            _ => unimplemented!(),
        };

        let div_shifted = self.div >> bit_shift_count;
        let previous_div_shifted = prev_div >> bit_shift_count;

        let timer_update = div_shifted != previous_div_shifted;

        if !timer_update {
            return None;
        }

        self.tima += 1;

        if self.tima < 0xFF {
            return None;
        }
        self.tima = self.tma;

        return Some(InterruptType::TIMER);
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

#[cfg(test)]
mod tests {
    use crate::cpu::context::InterruptType;

    use super::Timer;
    #[test]
    fn timer_ticks_disabled() {
        let mut timer = Timer::new();
        timer.tac = 0b11;
        test_tima_update(1, &timer, 0);
    }

    fn test_tima_update(ticks: u16, old_timer: &Timer, expected_tima_increment: u8) {
        let mut timer = old_timer.clone();

        for tick in 1..ticks {
            assert!(timer.timer_tick().is_none());

            let expected = Timer {
                div: old_timer.div + tick,
                ..*old_timer
            };
            assert_eq!(timer, expected, "tick={}", tick);
        }
        assert!(timer.timer_tick().is_none());

        let expected = Timer {
            div: old_timer.div + ticks,
            tima: expected_tima_increment + old_timer.tima,
            ..*old_timer
        };
        assert_eq!(timer, expected, "tick={}", ticks);
    }

    fn test_interrupt(ticks: usize, timer: &mut Timer, expected_interrupt: Option<InterruptType>) {
        let old_timer = timer.clone();

        for _ in 1..ticks {
            assert!(timer.timer_tick().is_none());
        }

        assert!(timer.timer_tick().eq(&expected_interrupt));

        let expected = Timer {
            div: ((old_timer.div as usize) + ticks) as u16,
            tima: old_timer.tma,
            ..old_timer
        };
        assert_eq!(*timer, expected);
    }

    #[test]
    fn tima_div_16() {
        let mut timer = Timer::new();
        timer.tac = 0b101;

        test_tima_update(15, &timer, 0);
    }

    #[test]
    fn tima_div_64() {
        let mut timer = Timer::new();
        timer.tac = 0b110;

        test_tima_update(64, &timer, 1);
    }
    #[test]
    fn tima_div_256() {
        let mut timer = Timer::new();
        timer.tac = 0b111;

        test_tima_update(256, &timer, 1);
    }
    #[test]
    fn tima_div_1024() {
        let mut timer = Timer::new();
        timer.tac = 0b100;

        test_tima_update(1024, &timer, 1);
    }
    #[test]
    fn tima_div_16_interrupt() {
        let mut timer = Timer::new();
        timer.tac = 0b101;

        let mut ticks = 0x10 * 0xFF;
        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));

        timer.tma = 0xF0;
        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));

        ticks = 0x10 * 0x0F;
        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));
    }

    #[test]
    fn tima_div_64_interrupt() {
        let mut timer = Timer::new();
        timer.tac = 0b110;

        let mut ticks = 64 * 0xFF;

        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));

        timer.tma = 0xF0;
        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));

        ticks = 64 * 0x0F;
        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));
    }
    #[test]
    fn tima_div_256_interrupt() {
        let mut timer = Timer::new();
        timer.tac = 0b111;

        let mut ticks = 256 * 0xFF;

        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));
        timer.tma = 0xF0;
        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));

        ticks = 256 * 0x0F;
        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));
    }
    #[test]
    fn tima_div_1024_interrupt() {
        let mut timer = Timer::new();
        timer.tac = 0b100;

        let mut ticks = 1024 * 0xFF;

        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));
        timer.tma = 0xF0;
        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));

        ticks = 1024 * 0x0F;
        test_interrupt(ticks, &mut timer, Some(InterruptType::TIMER));
    }
}
