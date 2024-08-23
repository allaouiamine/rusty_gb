use std::sync::{Arc, Mutex};

use crate::{cpu::types::InterruptType, dma::DMA};

const DEFAULT_COLORS: [u32; 4] = [0xFFFFFFFF, 0xFFAAAAAA, 0xFF555555, 0xFF000000];

#[derive(Default)]
pub struct LCD {
    lcd_control: u8,   // LCDC 0xFF40
    lcd_status: u8,    // STAT 0xFF41
    pub scroll_y: u8,      // SCY 0xFF42
    pub scroll_x: u8,      // SCX 0xFF43
    pub ly: u8,            // LY 0xFF44
    ly_compare: u8,    // LYC 0xFF45
    dma: u8,           // DMA 0xFF46
    bg_palette: u8,    // BGP 0xFF47
    obj_palette_0: u8, // OBP0 0xFF48
    obj_palette_1: u8, // OBP1 0xFF49
    window_y: u8,      // WY 0xFF4A
    window_x: u8,      // WX 0xFF4B

    pub dma_state: Arc<Mutex<DMA>>,
    pub bg_colors: [u32; 4],
    sprite_0_colors: [u32; 4],
    sprite_1_colors: [u32; 4],
}

impl LCD {
    pub fn new(dma_state: Arc<Mutex<DMA>>) -> Self {
        Self {
            lcd_control: 0x91,
            bg_palette: 0xFC,
            obj_palette_0: 0xFF,
            obj_palette_1: 0xFF,
            dma_state,
            bg_colors: DEFAULT_COLORS,
            sprite_0_colors: DEFAULT_COLORS,
            sprite_1_colors: DEFAULT_COLORS,
            ..Default::default()
        }
    }

    pub fn lcd_status_mode_get(&self) -> LCDMode {
       (self.lcd_status & 0b11).into()
    }

    pub fn lcd_status_mode_set(&mut self, mode: LCDMode) {
        let mode_value = mode as u8;
        // Clear the last two bits and OR with the new mode
        self.lcd_status = (self.lcd_status & 0xFC) | mode_value;
    }

    pub fn lcd_interrupt_status_get(&self, status: LCDStatusSelect) -> bool {
        let status_int = status as u8;
        self.lcd_status & status_int != 0
    }

    pub fn lcd_ly_reset(&mut self) {
        self.ly = 0;
    }

    pub fn lcd_ly_increment(&mut self) -> Option<InterruptType> {
        self.ly += 1;
        if self.ly == self.ly_compare {
            self.lcd_status |= 0b100;

            if self.lcd_status & (1 << 6) != 0 {
                println!("LYC=LY interrupt");
                return Some(InterruptType::LCDStat);
            }
        } else {
            self.lcd_status &= 0b11111011;
        }
        None
    }

    pub fn lcd_read(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcd_control,
            0xFF41 => self.lcd_status,
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.ly,
            0xFF45 => self.ly_compare,
            0xFF46 => self.dma,
            0xFF47 => self.bg_palette,
            0xFF48 => self.obj_palette_0,
            0xFF49 => self.obj_palette_1,
            0xFF4A => self.window_y,
            0xFF4B => self.window_x,
            _ => {
                println!("UNSUPPORTED lcd_read({:#02X}) - LCD", address);
                0
            }
        }
    }

    pub fn lcd_write(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => self.lcd_control = value,
            0xFF41 => self.lcd_status = value,
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF44 => {
                println!(
                    "lcd_write({:#02X}, {:#02X}) - LY: {:02X}",
                    address, value, self.ly
                );
                self.ly = value
            }
            0xFF45 => self.ly_compare = value,
            0xFF46 => {
                self.dma = value;
                println!("DMA START: {:#02X}", value);
                self.dma_state.lock().unwrap().dma_start(value);
            }
            0xFF47 => {
                self.bg_colors = self.get_updated_palette(value);
                self.bg_palette = value;
            }
            0xFF48 => {
                // The last two bits are ignored because index 0 is transparent for objects
                self.sprite_0_colors = self.get_updated_palette(value & 0xFC); // set The last 2
                                                                               // bits to 0
                self.obj_palette_0 = value;
            }
            0xFF49 => {
                // The last two bits are ignored because index 0 is transparent for objects
                self.sprite_1_colors = self.get_updated_palette(value & 0xFC); // set The last 2
                                                                               // bits to 0
                self.obj_palette_1 = value;
            }
            0xFF4A => self.window_y = value,
            0xFF4B => self.window_x = value,
            _ => {
                println!(
                    "UNSUPPORTED lcd_write({:#02X}, {:#02X}) - LCD",
                    address, value
                );
            }
        }
    }

    fn get_updated_palette(&mut self, value: u8) -> [u32; 4] {
        let mut palette = [0; 4];
        for i in 0..4 {
            let color = (value >> (i * 2)) & 0b11;
            palette[i] = DEFAULT_COLORS[color as usize];
        }
        palette
    }

    fn get_lcd_control_bit(&self, bit: u8) -> bool {
        self.lcd_control & (1 << bit) != 0
    }

    pub fn get_bg_window_enable(&self) -> bool {
        self.get_lcd_control_bit(0)
    }

    pub fn get_background_data_area(&self) -> u16 {
        if self.get_lcd_control_bit(4) {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn get_background_map_area(&self) -> u16 {
        if self.get_lcd_control_bit(3) {
            0x9C00
        } else {
            0x9800
        }
    }
}

pub enum LCDMode {
    HorizentalBlank = 0,
    VerticalBlank = 1,
    OAMSearch = 2,
    PixelTransfer = 3,
}

impl From<u8> for LCDMode {
    fn from(value: u8) -> Self {
        match value {
            0 => LCDMode::HorizentalBlank,
            1 => LCDMode::VerticalBlank,
            2 => LCDMode::OAMSearch,
            3 => LCDMode::PixelTransfer,
            _ => panic!("Invalid LCDMode value: {}", value),
        }
    }
}


pub enum LCDStatusSelect {
    HBlank = (1 << 3),
    VBlank = (1 << 4),
    OAM = (1 << 5),
    LYC = (1 << 6),
}
