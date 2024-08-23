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

use std::sync::{Arc, Mutex};

use crate::{
    cartridge::Cartridge, cpu::types::InterruptType, dma::DMA, graphics::PPU, io::IO,
    ram::RamContext,
};

pub trait Bus: Send + Sync {
    fn bus_read(&self, address: u16) -> u8;
    fn bus_write(&mut self, address: u16, value: u8);
    fn dbg_update(&mut self);
    fn dbg_print(&self);
    fn timer_tick(&mut self) -> Option<InterruptType>;
    fn ppu_tick(&mut self) -> Vec<InterruptType>;
    fn dma_tick(&mut self) -> bool;
    fn get_video_buffer(&self) -> &Vec<u32>;
}

pub struct GbBus {
    cartridge: Cartridge,
    ram: RamContext,
    pub io: IO,
    interrupt_enable_register: u8,

    pub ppu: PPU,
    dma: Arc<Mutex<DMA>>,

    dbg_message: [u8; 1024],
    dbg_message_size: usize,
}

impl Bus for GbBus {
    fn bus_read(&self, address: u16) -> u8 {
        if address < 0x8000 {
            self.cartridge.cart_read(address)
        } else if address < 0xA000 {
            // character map data
            self.ppu.ppu_vram_read(address)
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
            if self.dma.lock().unwrap().dma_is_transferring() {
                0xFF
            } else {
                self.ppu.oam_read(address)
            }
        } else if address < 0xFF00 {
            // Not Usable	Nintendo says use of this area is prohibited
            0
        } else if address >= 0xFF40 && address <= 0xFF4B {
            // LCD registers. This is a special case here because the LCD is written as part
            // of the PPU to avoid multiple mutable borrows or mutexes.
            self.ppu.lcd_read(address)
        } else if address < 0xFF80 {
            // IO registers
            self.io.io_read(address)
        } else if address == 0xFFFF {
            // CPU interrupt enable register (IE)
            self.interrupt_enable_register
        } else {
            // High RAM (HRAM)
            self.ram.hram_read(address)
        }
    }

    fn bus_write(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            // ROM data
            self.cartridge.cart_write(address, value);
        } else if address < 0xA000 {
            // character map data
            self.ppu.ppu_vram_write(address, value);
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
            if self.dma.lock().unwrap().dma_is_transferring() {
                return;
            }
            self.ppu.oam_write(address, value, false);
        } else if address < 0xFF00 {
            // Not Usable	Nintendo says use of this area is prohibited
        } else if address >= 0xFF40 && address <= 0xFF4B {
            // LCD registers. This is a special case here because the LCD is written as part
            // of the PPU to avoid multiple mutable borrows or mutexes.
            self.ppu.lcd_write(address, value)
        } else if address < 0xFF80 {
            // IO registers
            self.io.io_write(address, value)
        } else if address == 0xFFFF {
            // CPU interrupt enable register (IE)
            self.interrupt_enable_register = value;
        } else {
            // High RAM (HRAM)
            self.ram.hram_write(address, value);
        }
    }
    fn dbg_update(&mut self) {
        if self.bus_read(0xFF02) == 0x81 {
            self.dbg_message[self.dbg_message_size] = self.bus_read(0xFF01);

            self.dbg_message_size += 1;

            self.bus_write(0xFF02, 0);
        }
    }

    fn dbg_print(&self) {
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
            Err(_) => println!("DBG: Error parsing message"),
        }
    }

    fn timer_tick(&mut self) -> Option<InterruptType> {
        self.io.timer.timer_tick()
    }

    fn dma_tick(&mut self) -> bool {
        let dma = &mut self.dma.lock().unwrap();
        if !dma.active {
            return false;
        }

        if dma.start_delay > 0 {
            dma.start_delay -= 1;
            return false;
        }

        let destination_address = dma.byte as u16;
        let source_address = ((dma.value as u16) * 0x100) + (dma.byte as u16);
        let value = self.bus_read(source_address);

        self.ppu.oam_write(destination_address, value, true);

        dma.byte += 1;

        dma.active = dma.byte < 0xA0;

        if !dma.dma_is_transferring() {
            true
        } else {
            false
        }
    }

    fn ppu_tick(&mut self) -> Vec<InterruptType> {
        self.ppu.tick()
    }

    fn get_video_buffer(&self) -> &Vec<u32> {
        &self.ppu.video_buffer
    }
}

impl GbBus {
    pub fn new(rom_file: String) -> Self {
        println!("Starting gb emulator with rom file: {}", rom_file);

        // load the cartridge
        let cartridge = Cartridge::load(rom_file);

        let ram: RamContext = RamContext::new();
        let io = IO::new();

        // DMA can be used directly from the bus or from the PPU/LCD
        // Without the interior mutability of the Mutex, it would be
        // very difficult to manage the mutable borrows of the DMA
        let dma = Arc::new(Mutex::new(DMA::new()));
        let ppu = PPU::new(Arc::clone(&dma));

        Self {
            cartridge,
            ram,
            io,
            interrupt_enable_register: 0,
            ppu,
            dma,
            dbg_message: [0; 1024],
            dbg_message_size: 0,
        }
    }

    pub fn fetch_tile(&self, tile_number: usize) -> [u8; 16] {
        if tile_number > 384 {
            panic!("Maximum tiles supported: {}", 384);
        }
        let tile_address = 0x8000 + (tile_number * 16) as u16;

        let mut tile_array: [u8; 16] = [0; 16];
        for i in 0..16 {
            tile_array[i] = self.bus_read(tile_address + (i as u16));
        }
        tile_array
    }
}
