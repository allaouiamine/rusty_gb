use std::{
    sync::{Arc, Mutex},
    usize,
};

use minifb::{Scale, ScaleMode, Window, WindowOptions};

use crate::{bus::Bus, graphics::constants::{X_RESOLUTION, Y_RESOLUTION}};

const TILE_COLORS: [u32; 4] = [0xFFFFFF, 0xAAAAAA, 0x555555, 0x000000];
const DEFAULT_BG_COLOR: u32 = 0x113F11;

const DBG_START_ADDRESS: u16 = 0x8000;

pub struct UI {
    pub main_window: Window,
    pub dbg_window: Window,
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub bus: Arc<Mutex<dyn Bus>>,
}

impl UI {
    const TILE_WIDTH: usize = 8;
    const TILE_LENGTH: usize = 8;
    pub fn new(
        width: usize,
        height: usize,
        scale_factor: Scale,
        bus: Arc<Mutex<dyn Bus>>,
    ) -> anyhow::Result<Self> {
        let buffer = vec![DEFAULT_BG_COLOR; width * height];
        let mut options = WindowOptions::default();
        options.scale_mode = ScaleMode::AspectRatioStretch;
        options.scale = scale_factor;
        let mut dbg_window = Window::new("Debug Window - ESC to exit", width, height, options)?;
        let mut main_window = Window::new("Gameboy Emulator", X_RESOLUTION, Y_RESOLUTION, options.clone())?;

        dbg_window.set_background_color(0x11, 0x3F, 0x11);
        main_window.set_background_color(0x11, 0x3F, 0x11);
        dbg_window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // 60 FPS
        main_window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // 60 FPS
        Ok(Self {
            main_window,
            dbg_window,
            buffer,
            width,
            height,
            bus,
        })
    }

    fn align_window(&mut self) {
        // let (x, y) = self.main_window.get_position();
        //self.dbg_window.set_position(x + X_RESOLUTION as isize, y);
    }

    fn update_dbg_window(&mut self) {
        for tile_number in 0..384 {
            self.draw_tile(tile_number as usize);
        }
    }
    pub fn combine_tile_bytes_to_tile_line(byte1: u8, byte2: u8) -> [u32; 8] {
        let mut line: [u32; 8] = [0; 8];
        for bit in 0..8 {
            let hi = ((byte1 as u16) & (1 << bit)) << 1;
            let lo = (byte2 as u16) & (1 << bit);
            let color_index = ((hi | lo) >> bit) as usize;

            line[bit] = TILE_COLORS[color_index];
        }
        line.reverse();
        line
    }

    fn draw_tile(&mut self, tile_number: usize) {
        let bus = self.bus.lock().unwrap();
        let tile_address = DBG_START_ADDRESS + (tile_number as u16 * 16);
        let mut buffer_adjust = (tile_number / 16) * self.width * (Self::TILE_WIDTH - 1);
        for y in 0..Self::TILE_LENGTH {
            let byte_1 = bus.bus_read(tile_address + (y as u16* 2));
            let byte_2 = bus.bus_read(tile_address + (y as u16* 2) + 1);

            let start_index = tile_number * Self::TILE_WIDTH + buffer_adjust;
            let end_index = start_index + 8;
            self.buffer[start_index..end_index]
                .clone_from_slice(&Self::combine_tile_bytes_to_tile_line(byte_1, byte_2));
            buffer_adjust += self.width;
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while self.dbg_window.is_open() && !self.dbg_window.is_key_down(minifb::Key::Escape) {
            self.align_window();
            self.dbg_window
                .update_with_buffer(&self.buffer, self.width, self.height)?;
            self.main_window.update_with_buffer(self.bus.lock().unwrap().get_video_buffer(), X_RESOLUTION, Y_RESOLUTION)?;
            self.update_dbg_window();
        }
        anyhow::bail!("UI closed");
    }
}
