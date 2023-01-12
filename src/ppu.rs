#[derive(Clone, Copy)]
pub struct Sprite {
    y: u8,
    x: u8,
    tile_index: u8,
    flags: u8,
}

impl Sprite {
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

/*
 Bit7   BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
 Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
 Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
 Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
 Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
 Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
*/

pub struct PPU {
    oam_ram: [Sprite; 40],
    vram: [u8; 0x2000],
}

impl PPU {
    pub fn new() -> Self {
        Self {
            oam_ram: [Sprite::new(); 40],
            vram: [0; 0x2000],
        }
    }

    pub fn oam_read(&self, address: u16, relative_address: bool) -> u8 {
        let (sprite_index, offset) = translate_oam_address(address, relative_address).unwrap();
        *(&self.oam_ram[sprite_index].get_field_from_offset(offset))
    }
    pub fn oam_write(&mut self, address: u16, value: u8, relative_address: bool) {
        let (sprite_index, offset) = translate_oam_address(address, relative_address).unwrap();
        let sprite = &mut self.oam_ram[sprite_index];
        sprite.set_field_from_offset(offset, value);
    }
    pub fn ppu_vram_read(&self, address: u16) -> u8 {
        self.vram[address as usize - 0x8000]
    }
    pub fn ppu_vram_write(&mut self, address: u16, value: u8) {
        self.vram[address as usize - 0x8000] = value;
    }
}

pub fn translate_oam_address(address: u16, relative_address: bool) -> Result<(usize, u8), ()> {
    let mut translated_address = address;

    if relative_address && translated_address >= 0xA0 {
        return Err(());
    }

    if !relative_address {
        translated_address -= 0xFE00;
    }
    let sprite_index = translated_address >> 2;
    let offset = translated_address & 0b11;
    Ok((sprite_index as usize, offset as u8))
}

#[cfg(test)]
mod tests {
    use super::translate_oam_address;
    #[test]
    fn translate_oam_address_ok() {
        assert_eq!(translate_oam_address(0xFE00, false).unwrap(), (0, 0));
        assert_eq!(translate_oam_address(0xFE16, false).unwrap(), (5, 2));
        assert_eq!(translate_oam_address(0xFE15, false).unwrap(), (5, 1));
        assert_eq!(translate_oam_address(0xFE14, false).unwrap(), (5, 0));
        assert_eq!(translate_oam_address(0xFE13, false).unwrap(), (4, 3));

        assert_eq!(
            translate_oam_address(0xFE00, false).unwrap(),
            translate_oam_address(0x00, true).unwrap()
        );
        assert_eq!(
            translate_oam_address(0xFE16, false).unwrap(),
            translate_oam_address(0x16, true).unwrap()
        );
        assert_eq!(
            translate_oam_address(0xFE15, false).unwrap(),
            translate_oam_address(0x15, true).unwrap()
        );
        assert_eq!(
            translate_oam_address(0xFE14, false).unwrap(),
            translate_oam_address(0x14, true).unwrap()
        );
        assert_eq!(
            translate_oam_address(0xFE13, false).unwrap(),
            translate_oam_address(0x13, true).unwrap()
        );
    }

    #[test]
    fn translate_oam_address_err() {
        assert!(translate_oam_address(0xA0, true).is_err());
        assert!(translate_oam_address(0xA1, true).is_err());
    }
}
