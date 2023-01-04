/*
Start	End	Description	Notes
0000	3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
4000	7FFF	16 KiB ROM Bank 01~NN	From cartridge, switchable bank via mapper (if any)
8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
A000	BFFF	8 KiB External RAM	From cartridge, switchable bank if any
C000	CFFF	4 KiB Work RAM (WRAM)
D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1~7
E000	FDFF	Mirror of C000~DDFF (ECHO RAM)	Nintendo says use of this area is prohibited.
FE00	FE9F	Sprite attribute table (OAM)
FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited
FF00	FF7F	I/O Registers
FF80	FFFE	High RAM (HRAM)
FFFF	FFFF	Interrupt Enable register (IE)
*/

use crate::{cartridge::Cartridge, io::IO, ram::RamContext};

// use crate::ram::RamContext;

pub struct Bus<'a> {
    cartridge: Cartridge<'a>,
    ram: RamContext,
    pub io: IO,
    interrupt_enable_register: u8,

    dbg_message: [u8; 1024],
    dbg_message_size: usize,
}

impl<'a> Bus<'a> {
    pub fn new(rom_file: &'a str) -> Self {
        println!("Starting gb emulator with rom file: {}", rom_file);

        // load the cartridge
        let cartridge = Cartridge::load(rom_file);

        // initialize the RAM
        let ram: RamContext = RamContext::new();

        let io = IO::new();

        Self {
            cartridge,
            ram,
            io,
            interrupt_enable_register: 0,
            dbg_message: [0; 1024],
            dbg_message_size: 0,
        }
    }

    pub fn dbg_update(&mut self) {
        if self.bus_read(0xFF02) == 0x81 {
            self.dbg_message[self.dbg_message_size] = self.bus_read(0xFF01);

            self.dbg_message_size += 1;

            self.bus_write8(0xFF02, 0);
        }
    }

    pub fn dbg_print(&self) {
        let mut message: Vec<u8> = Vec::new();

        for c in self.dbg_message {
            if c != 0 {
                message.push(c);
            }
        }

        if message.len() == 0 {
            return;
        }
        match String::from_utf8(message) {
            Ok(m) => println!("DBG: {}", m),
            Err(_) => {}
        }
    }

    pub fn get_ie_register(&self) -> u8 {
        self.interrupt_enable_register
    }

    fn set_ie_register(&mut self, value: u8) {
        self.interrupt_enable_register = value;
    }

    pub fn bus_read(&self, address: u16) -> u8 {
        if address < 0x8000 {
            // RM data
            self.cartridge.cart_read(address)
        } else if address < 0xA000 {
            // character map data
            println!(
                "UNSUPPORTED bus_read({:#02X}) - character map data",
                address
            );
            // unimplemented!();
            0
        } else if address < 0xC000 {
            // cartridge RAM
            self.cartridge.cart_read(address)
        } else if address < 0xE000 {
            // working ram WRAM
            self.ram.wram_read(address)
        } else if address < 0xFE00 {
            // Mirror of C000~DDFF (ECHO RAM)	Nintendo says use of this area is prohibited.
            0
        } else if address < 0xFEA0 {
            // Sprite attribute table (OAM)
            println!("UNSUPPORTED bus_read({:#02X}) - OAM", address);
            0
            // unimplemented!();
        } else if address < 0xFF00 {
            // Not Usable	Nintendo says use of this area is prohibited
            0
        } else if address < 0xFF80 {
            // IO registers
            self.io.io_read(address)
        } else if address == 0xFFFF {
            // CPU interrupt enable register (IE)
            self.get_ie_register()
        } else {
            // High RAM (HRAM)
            self.ram.hram_read(address)
        }
    }

    pub fn bus_read16(&self, address: u16) -> u16 {
        let lo = self.bus_read(address) as u16;
        let hi = self.bus_read(address + 1) as u16;
        lo | (hi << 8)
    }

    pub fn bus_write8(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            // ROM data
            self.cartridge.cart_write(address, value);
        } else if address < 0xA000 {
            // character map data
            println!(
                "UNSUPPORTED bus_write({:#02X}, {:#02X}) - character map data",
                address, value
            );
            // unimplemented!()
        } else if address < 0xC000 {
            // cartridge RAM
            self.cartridge.cart_write(address, value);
        } else if address < 0xE000 {
            // working ram WRAM
            self.ram.wram_write(address, value);
        } else if address < 0xFE00 {
            // Mirror of C000~DDFF (ECHO RAM)	Nintendo says use of this area is prohibited.
        } else if address < 0xFEA0 {
            // Sprite attribute table (OAM)
            println!("UNSUPPORTED bus_write({:#02X}, {:#02X})", address, value);
            unimplemented!();
        } else if address < 0xFF00 {
            // Not Usable	Nintendo says use of this area is prohibited
        } else if address < 0xFF80 {
            // IO registers
            self.io.io_write(address, value)
        } else if address == 0xFFFF {
            // CPU interrupt enable register (IE)
            self.set_ie_register(value);
        } else {
            // High RAM (HRAM)
            self.ram.hram_write(address, value);
        }
    }

    pub fn bus_write16(&mut self, address: u16, value: u16) {
        self.bus_write8(address, value as u8);
        self.bus_write8(address + 1, (value >> 8) as u8);
    }
}
