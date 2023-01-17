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

/*
 Bit7   BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
 Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
 Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
 Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
 Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
 Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
*/

pub struct PPU {
    oam_ram: [OamEntry; 40],
    vram: [u8; 0x2000],
}

impl PPU {
    pub fn new() -> Self {
        Self {
            oam_ram: [OamEntry::new(); 40],
            vram: [0; 0x2000],
        }
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
}

pub fn translate_oam_address(address: u16, dma: bool) -> (usize, u8) {
    let translated_address = if !dma {
        address - 0xFE00
    } else {
        if address > 0x9F {
            panic!("cannot write address: {:02X} through DMA", address);
        }
        address
    };

    let oam_entry_index = translated_address >> 2;
    let offset = translated_address & 0b11;
    (oam_entry_index as usize, offset as u8)
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
