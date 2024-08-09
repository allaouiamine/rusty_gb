#[derive(Clone, Copy)]
pub struct OamEntry {
    y: u8,
    x: u8,
    tile_index: u8,
    flags: u8,
}

impl OamEntry {
    pub fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            tile_index: 0,
            flags: 0,
        }
    }
    pub fn get_field_from_offset(&self, offset: u8) -> u8 {
        match offset & 0b11 {
            0 => self.y,
            1 => self.x,
            2 => self.tile_index,
            3 => self.flags,
            _ => unimplemented!(),
        }
    }
    pub fn set_field_from_offset(&mut self, offset: u8, value: u8) {
        match offset & 0b11 {
            0 => {
                self.y = value;
            }
            1 => {
                self.x = value;
            }
            2 => {
                self.tile_index = value;
            }
            3 => {
                self.flags = value;
            }
            _ => {
                unimplemented!();
            }
        };
    }
}
