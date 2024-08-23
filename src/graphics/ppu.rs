use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{
    cpu::types::InterruptType,
    dma::DMA,
    io::{
        lcd::{LCDMode, LCDStatusSelect},
        LCD,
    },
};

use super::{
    constants::{X_RESOLUTION, Y_RESOLUTION},
    oam_entry::OamEntry,
    state_machine::PPUStateMachine,
};

pub struct PPU {
    oam_ram: [OamEntry; 40],
    vram: [u8; 0x2000],

    pub current_frame: usize,
    pub line_ticks: usize,
    pub video_buffer: Vec<u32>,
    pub lcd: LCD,
    pub pixel_fifo: PixelFifo,
}

impl PPU {
    pub fn new(dma_state: Arc<Mutex<DMA>>) -> Self {
        let mut lcd = LCD::new(dma_state);
        lcd.lcd_status_mode_set(LCDMode::OAMSearch);
        Self {
            oam_ram: [OamEntry::new(); 40],
            vram: [0; 0x2000],
            current_frame: 0,
            line_ticks: 0,
            video_buffer: vec![0; X_RESOLUTION * Y_RESOLUTION],
            lcd,
            pixel_fifo: PixelFifo::new(),
        }
    }

    pub fn get_background_data_area(&self) -> u16 {
        self.lcd.get_background_data_area()
    }
    pub fn get_background_map_area(&self) -> u16 {
        self.lcd.get_background_map_area()
    }

    pub fn ly_increment(&mut self) -> Option<InterruptType> {
        self.lcd.lcd_ly_increment()
    }

    pub fn ly_reset(&mut self) {
        self.lcd.lcd_ly_reset();
    }

    pub fn lcd_status_mode_set(&mut self, mode: LCDMode) {
        self.lcd.lcd_status_mode_set(mode);
    }

    pub fn lcd_interrupt_status_get(&self, status: LCDStatusSelect) -> bool {
        self.lcd.lcd_interrupt_status_get(status)
    }

    pub fn tick(&mut self) -> Vec<InterruptType> {
        self.line_ticks += 1;
        self.ppu_step()
    }

    pub fn oam_read(&self, address: u16) -> u8 {
        let (sprite_index, offset) = translate_oam_address(address, false);
        *(&self.oam_ram[sprite_index].get_field_from_offset(offset))
    }

    pub fn oam_write(&mut self, address: u16, value: u8, dma: bool) {
        let (sprite_index, offset) = translate_oam_address(address, dma);
        let sprite = &mut self.oam_ram[sprite_index];
        sprite.set_field_from_offset(offset, value);
    }
    pub fn ppu_vram_read(&self, address: u16) -> u8 {
        self.vram[address as usize - 0x8000]
    }
    pub fn ppu_vram_write(&mut self, address: u16, value: u8) {
        self.vram[address as usize - 0x8000] = value;
    }

    pub fn lcd_read(&self, address: u16) -> u8 {
        self.lcd.lcd_read(address)
    }

    pub fn lcd_write(&mut self, address: u16, value: u8) {
        self.lcd.lcd_write(address, value);
    }
}

pub fn translate_oam_address(address: u16, dma: bool) -> (usize, u8) {
    let translated_address = if !dma {
        address - 0xFE00
    } else {
        if address > 0x9F {
            panic!("cannot read or write address: {:02X} through DMA", address);
        }
        address
    };

    let oam_entry_index = translated_address >> 2;
    let offset = translated_address & 0b11;
    (oam_entry_index as usize, offset as u8)
}

#[derive(Clone, Copy)]
pub enum PixelFetcherState {
    Tile,
    DataLow,
    DataHigh,
    Idle,
    Push,
}

pub struct PixelFifo {
    pub fetcher_state: PixelFetcherState,
    pub fifo: VecDeque<u32>,
    pub line_x: u8,
    pub pushed_x: u8,
    pub fetched_x: u8,
    pub bg_window_fetch_data: [u8; 3],
    pub fetch_entry_data: [u8; 6], //OAM
    pub map_y: u8,
    pub map_x: u8,
    pub tile_y: u8,
    pub fifo_x: u8,
}

impl PixelFifo {
    pub fn new() -> Self {
        Self {
            fetcher_state: PixelFetcherState::Tile,
            fifo: VecDeque::with_capacity(16),
            line_x: 0,
            pushed_x: 0,
            fetched_x: 0,
            bg_window_fetch_data: [0; 3],
            fetch_entry_data: [0; 6],
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
        }
    }

    pub fn reset_oam(&mut self) {
        self.fetcher_state = PixelFetcherState::Tile;
        self.line_x = 0;
        self.pushed_x = 0;
        self.fetched_x = 0;
        self.fifo_x = 0;
    }

    pub fn push(&mut self, pixel: u32) {
        self.fifo.push_back(pixel);
    }

    pub fn pop(&mut self) -> Option<u32> {
        self.fifo.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::translate_oam_address;
    #[test]
    fn translate_oam_address_ok() {
        assert_eq!(translate_oam_address(0xFE00, false), (0, 0));
        assert_eq!(translate_oam_address(0xFE16, false), (5, 2));
        assert_eq!(translate_oam_address(0xFE15, false), (5, 1));
        assert_eq!(translate_oam_address(0xFE14, false), (5, 0));
        assert_eq!(translate_oam_address(0xFE13, false), (4, 3));

        assert_eq!(
            translate_oam_address(0xFE00, false),
            translate_oam_address(0x00, true)
        );
        assert_eq!(
            translate_oam_address(0xFE16, false),
            translate_oam_address(0x16, true)
        );
        assert_eq!(
            translate_oam_address(0xFE15, false),
            translate_oam_address(0x15, true)
        );
        assert_eq!(
            translate_oam_address(0xFE14, false),
            translate_oam_address(0x14, true)
        );
        assert_eq!(
            translate_oam_address(0xFE13, false),
            translate_oam_address(0x13, true)
        );
    }

    // #[test]
    // fn translate_oam_address_err() {
    //     assert!(translate_oam_address(0xA0, true).is_err());
    //     assert!(translate_oam_address(0xA1, true).is_err());
    // }
}
