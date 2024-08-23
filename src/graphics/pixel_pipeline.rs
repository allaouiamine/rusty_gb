use super::{constants::X_RESOLUTION, ppu::PixelFetcherState, PPU};

pub trait PixelPipelineExt {
    fn pipeline_push_pixel(&mut self);
    fn pipeline_process(&mut self);
    fn pipeline_fetch(&mut self);
    fn pipeline_fifo_reset(&mut self);
    fn pipeline_fifo_add(&mut self) -> bool;
}

impl PixelPipelineExt for PPU {
    fn pipeline_process(&mut self) {
        let map_y;
        let map_x;
        let tile_y;
        map_y = self.lcd.ly + self.lcd.scroll_y;
        map_x = self.pixel_fifo.fetched_x + self.lcd.scroll_x;
        tile_y = (map_y % 8) * 2;

        self.pixel_fifo.map_y = map_y;
        self.pixel_fifo.map_x = map_x;
        self.pixel_fifo.tile_y = tile_y;

        if self.line_ticks & 1 == 0 {
            self.pipeline_fetch()
        }
        self.pipeline_push_pixel();
    }

    fn pipeline_fetch(&mut self) {
        let fetcher_state = self.pixel_fifo.fetcher_state;
        match fetcher_state {
            PixelFetcherState::Tile => {
                let lcdc_bgw_enable = self.lcd.get_bg_window_enable();
                if lcdc_bgw_enable {
                    let tile_data = self.ppu_vram_read(
                        self.get_background_map_area()
                            + (self.pixel_fifo.map_x as u16 / 8)
                            + ((self.pixel_fifo.map_y as u16 / 8) * 32),
                    );
                    if self.get_background_data_area() == 0x8800 {
                        self.pixel_fifo.bg_window_fetch_data[0] = tile_data + 128;
                    } else {
                        self.pixel_fifo.bg_window_fetch_data[0] = tile_data;
                    }
                }

                self.pixel_fifo.fetcher_state = PixelFetcherState::DataLow;
                self.pixel_fifo.fetched_x += 8;
            }
            PixelFetcherState::DataLow => {
                let tile_data = self.ppu_vram_read(
                    self.get_background_data_area()
                        + (self.pixel_fifo.bg_window_fetch_data[0] as u16 * 16)
                        + self.pixel_fifo.tile_y as u16,
                );
                self.pixel_fifo.bg_window_fetch_data[1] = tile_data;
                self.pixel_fifo.fetcher_state = PixelFetcherState::DataHigh;
            }
            PixelFetcherState::DataHigh => {
                let tile_data = self.ppu_vram_read(
                    self.get_background_data_area()
                        + (self.pixel_fifo.bg_window_fetch_data[0] as u16 * 16)
                        + (self.pixel_fifo.tile_y as u16 + 1),
                );
                self.pixel_fifo.bg_window_fetch_data[2] = tile_data;
                self.pixel_fifo.fetcher_state = PixelFetcherState::Idle;
            }
            PixelFetcherState::Idle => {
                self.pixel_fifo.fetcher_state = PixelFetcherState::Push;
            }
            PixelFetcherState::Push => {
                if self.pipeline_fifo_add() {
                    self.pixel_fifo.fetcher_state = PixelFetcherState::Tile;
                } else {
                    self.pixel_fifo.fetcher_state = PixelFetcherState::Push
                }
            }
        }
    }

    fn pipeline_push_pixel(&mut self) {
        if self.pixel_fifo.fifo.len() > 8 {
            let pixel_data = self.pixel_fifo.pop().unwrap();

            let scroll_x = self.lcd.scroll_x;
            let ly = self.lcd.ly as usize;
            if self.pixel_fifo.line_x >= (scroll_x & 8) {
                let index = self.pixel_fifo.pushed_x as usize + (ly * X_RESOLUTION);
                self.video_buffer[index] = pixel_data;

                self.pixel_fifo.pushed_x += 1;
            }

            self.pixel_fifo.line_x += 1;
        }
    }

    fn pipeline_fifo_reset(&mut self) {
        self.pixel_fifo.fifo.clear()
    }

    fn pipeline_fifo_add(&mut self) -> bool {
        if self.pixel_fifo.fifo.len() > 8 {
            return false;
        }

        let scroll_y = self.lcd.scroll_y;

        let x = self.pixel_fifo.fetched_x as i8 - (8 - (scroll_y % 8) as i8);

        for i in 0..8 {
            let bit = 7 - i;
            let hi = ((self.pixel_fifo.bg_window_fetch_data[2] as u16) & (1 << bit)) << 1;
            let lo = (self.pixel_fifo.bg_window_fetch_data[1] as u16) & (1 << bit);
            let color = self.lcd.bg_colors[((hi | lo) >> bit) as usize];

            if x >= 0 {
                self.pixel_fifo.push(color);
                self.pixel_fifo.fifo_x += 1;
            }
        }
        true
    }
}
