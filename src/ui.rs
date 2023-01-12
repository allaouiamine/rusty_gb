use core::panic;

use crate::{bus::Bus, cpu::CpuContext};

use minifb::{Scale, ScaleMode, Window, WindowOptions};

const TILE_COLORS: [u32; 4] = [0xFFFFFF, 0xAAAAAA, 0x555555, 0x000000];
const TILE_COUNT: usize = 384;
const BYTES_PER_TILE: usize = 16;

const VRAM_TILE_ADDRESS: u16 = 0x8000;

#[derive(Copy, Clone)]
pub struct TilePosition {
    x: usize,
    y: usize,
}

pub struct UI {
    pub dbg_window: Window,
    width: usize,
    height: usize,
    pub buffer: Vec<u32>,
}

impl UI {
    pub fn new(width: usize, height: usize, scale_factor: Scale) -> Self {
        let buffer = vec![0x113F11; width * height];
        let mut options = WindowOptions::default();
        options.scale_mode = ScaleMode::AspectRatioStretch;
        options.scale = scale_factor;
        let mut dbg_window = Window::new("Debug Window - ESC to exit", width, height, options)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        dbg_window.set_background_color(0x11, 0x3F, 0x11);
        dbg_window.limit_update_rate(Some(std::time::Duration::from_micros(33300)));
        Self {
            dbg_window,
            buffer,
            width,
            height,
        }
    }

    pub fn update(&mut self, cpu: &CpuContext) {
        // check if tile is complete
        let address = cpu.last_written_address.unwrap() - 0x8000;
        if address & 0xF < 0xF {
            return;
        }
        let tile_number = (address >> 4) as usize;
        self.update_buffer_with_tile(tile_number, &cpu.bus);
        self.dbg_window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    fn update_buffer_with_tile(&mut self, tile_number: usize, bus: &Bus) {
        let tile_array = Self::fetch_tile(tile_number, bus);

        let mut s = String::new();
        let mut index = 0;
        while index < tile_array.len() {
            s = format!(
                "{} {:02X} {:02X}",
                s,
                tile_array[index],
                tile_array[index + 1]
            );
            index += 2
        }

        let mut position = TilePosition {
            x: (tile_number & 15) * 8,
            y: (tile_number >> 4) * 8,
        };

        for line_number in 0..8 {
            let tile_line = Self::combine_tile_bytes(
                tile_array[line_number * 2],
                tile_array[line_number * 2 + 1],
            );
            self.write_tile_line_to_buffer(tile_line, position);
            position.y += 1;
        }
    }

    fn write_tile_line_to_buffer(&mut self, tile_line: [u32; 8], position: TilePosition) {
        let mut x = position.x;
        for pixel in tile_line {
            let index = x + (position.y * self.width);
            self.buffer[index] = pixel;
            x += 1;
        }
    }

    fn fetch_tile(tile_number: usize, bus: &Bus) -> [u8; BYTES_PER_TILE] {
        if tile_number > TILE_COUNT {
            panic!("Maximum tiles supported: {}", TILE_COUNT);
        }
        let tile_address = VRAM_TILE_ADDRESS + (tile_number * BYTES_PER_TILE) as u16;

        let mut tile_array: [u8; 16] = [0; 16];
        for i in 0..16 {
            tile_array[i] = bus.bus_read(tile_address + (i as u16));
        }
        tile_array
    }

    pub fn combine_tile_bytes(byte1: u8, byte2: u8) -> [u32; 8] {
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
}

#[cfg(test)]
mod tests {
    use super::UI;
    #[test]
    pub fn combine_tile_bytes_ok() {
        let expected = [0, 0, 0, 0, 0, 0, 0, 0xFFFFFF];
        let line = UI::combine_tile_bytes(0xFE, 0xFE);
        for (index, pixel) in line.iter().enumerate() {
            assert_eq!(
                *pixel, expected[index],
                "index: {} - {:X} == {:X}",
                index, *pixel, expected[index]
            );
        }
    }
}
