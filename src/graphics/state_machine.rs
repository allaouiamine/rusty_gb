use crate::{
    cpu::types::InterruptType,
    io::lcd::{LCDMode, LCDStatusSelect},
};

use super::{
    constants::{LINES_PER_FRAME, TICKS_PER_LINE, Y_RESOLUTION},
    PPU,
};

pub trait PPUStateMachine {
    fn ppu_mode_oam(&mut self) -> Vec<InterruptType>;
    fn ppu_mode_transfer(&mut self) -> Vec<InterruptType>;
    fn ppu_mode_hblank(&mut self) -> Vec<InterruptType>;
    fn ppu_mode_vblank(&mut self) -> Vec<InterruptType>;
    fn ppu_step(&mut self) -> Vec<InterruptType>;
}

impl PPUStateMachine for PPU {
    fn ppu_mode_oam(&mut self) -> Vec<InterruptType> {
        if self.line_ticks >= 80 {
            self.lcd_status_mode_set(LCDMode::PixelTransfer);
        }
        vec![]
    }

    fn ppu_mode_transfer(&mut self) -> Vec<InterruptType> {
        if self.line_ticks >= 80 + 172 {
            self.lcd_status_mode_set(LCDMode::HorizentalBlank);
        }
        vec![]
    }

    fn ppu_mode_hblank(&mut self) -> Vec<InterruptType> {
        let mut interrupts = Vec::with_capacity(3);
        if self.line_ticks >= TICKS_PER_LINE {
            if let Some(interrupt) = self.ly_increment() {
                interrupts.push(interrupt);
            }

            if self.lcd_ly_get() >= Y_RESOLUTION as u8 {
                self.lcd_status_mode_set(LCDMode::VerticalBlank);

                interrupts.push(InterruptType::VBLANK);

                if self.lcd_interrupt_status_get(LCDStatusSelect::VBlank) {
                    interrupts.push(InterruptType::LCDStat);
                }

                self.current_frame += 1;
            } else {
                self.lcd_status_mode_set(LCDMode::OAMSearch);
            }

            self.line_ticks = 0;
        }
        interrupts
    }

    fn ppu_mode_vblank(&mut self) -> Vec<InterruptType> {
        let mut interrupts = Vec::with_capacity(1);
        if self.line_ticks >= TICKS_PER_LINE {
            if let Some(interrupt) = self.ly_increment() {
                interrupts.push(interrupt);
            }
            if self.lcd_ly_get() >= LINES_PER_FRAME {
                self.lcd_status_mode_set(LCDMode::OAMSearch);
                self.ly_reset();
            }

            self.line_ticks = 0;
        }
        interrupts
    }

    fn ppu_step(&mut self) -> Vec<InterruptType> {
        let lcd_status_mode = self.lcd.lock().unwrap().lcd_status_mode_get();
        match lcd_status_mode {
            LCDMode::HorizentalBlank => self.ppu_mode_hblank(),
            LCDMode::VerticalBlank => self.ppu_mode_vblank(),
            LCDMode::OAMSearch => self.ppu_mode_oam(),
            LCDMode::PixelTransfer => self.ppu_mode_transfer(),
        }
    }
}
