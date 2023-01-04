pub struct RamContext {
    wram: [u8; 0x2000],
    hram: [u8; 0x80],
}

impl RamContext {
    pub fn new() -> Self {
        Self {
            wram: [0; 0x2000],
            hram: [0; 0x80],
        }
    }

    pub fn wram_read(&self, address: u16) -> u8 {
        self.wram[translate_wram_address(address).unwrap() as usize]
    }

    pub fn wram_write(&mut self, address: u16, value: u8) {
        self.wram[translate_wram_address(address).unwrap() as usize] = value;
    }

    pub fn hram_read(&self, address: u16) -> u8 {
        self.hram[translate_hram_address(address).unwrap() as usize]
    }

    pub fn hram_write(&mut self, address: u16, value: u8) {
        self.hram[translate_hram_address(address).unwrap() as usize] = value;
    }
}

pub fn translate_wram_address(address: u16) -> Result<u16, ()> {
    let translated_address = address - 0xC000;

    if translated_address >= 0x2000 {
        println!("wram_write: INVALID WRAP ADDR {:#02X}", address);
        Err(())
    } else {
        Ok(translated_address)
    }
}

pub fn translate_hram_address(address: u16) -> Result<u16, ()> {
    let translated_address = address - 0xFF80;

    if translated_address >= 0x80 {
        println!("wram_write: INVALID WRAP ADDR {:#02X}", address);
        Err(())
    } else {
        Ok(translated_address)
    }
}
