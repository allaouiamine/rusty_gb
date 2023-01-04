use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::num::Wrapping;

use crate::cartridge::licensee_code::{NewLicenseeCode, OldLicenseeCode};

#[derive(Debug)]
#[allow(dead_code)]
struct RomHeader {
    entry_point: [u8; 0x4],
    nintendo_logo: [u8; 0x30],
    title: [u8; 0x10],

    manufacturer_code: [u8; 0x4],
    cgb_flag: u8,
    new_licensee_code: u16,
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum: u16,
}

#[allow(dead_code)]
pub struct Cartridge<'rom> {
    filename: &'rom str,
    rom_size: usize,
    rom_data: Vec<u8>,
    rom_header: RomHeader,
}

impl<'rom> Cartridge<'rom> {
    pub fn load(filename: &'rom str) -> Cartridge<'rom> {
        let mut rom_file = File::open(filename).unwrap();
        rom_file.seek(SeekFrom::End(0)).unwrap();
        let rom_size = rom_file.stream_position().unwrap() as usize;
        println!("Found Rom Size: {}", rom_size);

        rom_file.rewind().unwrap();

        let mut rom_data: Vec<u8> = Vec::with_capacity(rom_size as usize);
        rom_file.read_to_end(&mut rom_data).unwrap();

        let rom_header = RomHeader::new(&rom_data);

        // dbg!(&rom_header);

        println!("Cartridge loaded:");
        println!("\t Title: {}", rom_header.title_name());
        println!("\t Type: {}", rom_header.cartridge_type);
        println!(
            "\t ROM Size: {} KB",
            32 * (1 << rom_header.rom_size) as usize
        );
        println!("\t RAM Size: {}", rom_header.ram_size);
        // let gbc_rom =  match rom_header.cgb_flag {
        //     0x80 => "Color Backward compatible",
        //     0xC0 => "Color Only",
        //     _ => "Unspecified"
        // };
        // println!("\t Color mode: {}", gbc_rom);
        println!(
            "\t Licensee Code: {}, {}, {}",
            rom_header.old_licensee_code,
            rom_header.new_licensee_code,
            rom_header.cartridge_licensee_name()
        );
        println!("\t ROM Version: {}", rom_header.mask_rom_version_number);

        let calculated_checksum = Cartridge::calculate_checksum(&rom_data);
        let checksum_status = if calculated_checksum == rom_header.header_checksum {
            "PASS"
        } else {
            "FAIL"
        };
        println!(
            "\t Checksum: calculated: {}, expected: {}, result: {}",
            calculated_checksum, rom_header.header_checksum, checksum_status
        );

        Cartridge {
            filename,
            rom_size,
            rom_data,
            rom_header,
        }
    }
    fn calculate_checksum(rom_data: &[u8]) -> u8 {
        let mut checksum = Wrapping(0u8);
        for byte in rom_data[0x0134..0x014D].iter() {
            let wrapped_byte = Wrapping(*byte);
            checksum = checksum - wrapped_byte - Wrapping(1u8);
        }
        checksum.0
    }

    pub fn cart_read(&self, address: u16) -> u8 {
        // For now we only support ROM ONLY cart type ...
        return self.rom_data[address as usize];
    }

    pub fn cart_write(&self, address: u16, value: u8) {
        // For now we only support ROM ONLY cart type ...
        println!("cart_write({:#02X}, {:#02X})", address, value);
        // unimplemented!();
    }
}

impl RomHeader {
    pub fn new(rom_data: &Vec<u8>) -> Self {
        let new_licensee_code: u16 = ((rom_data[0x144] as u16) << 8) | rom_data[0x145] as u16;
        let global_checksum = ((rom_data[0x14E] as u16) << 8) | rom_data[0x14F] as u16;
        Self {
            entry_point: rom_data[0x100..0x104].try_into().unwrap(),
            nintendo_logo: rom_data[0x104..0x134].try_into().unwrap(),
            title: rom_data[0x134..0x144].try_into().unwrap(),
            manufacturer_code: rom_data[0x13F..0x143].try_into().unwrap(),
            cgb_flag: rom_data[0x143],
            new_licensee_code: new_licensee_code,
            sgb_flag: rom_data[0x146],
            cartridge_type: rom_data[0x147],
            rom_size: rom_data[0x148],
            ram_size: rom_data[0x149],
            destination_code: rom_data[0x14A],
            old_licensee_code: rom_data[0x14B],
            mask_rom_version_number: rom_data[0x14C],
            header_checksum: rom_data[0x14D],
            global_checksum: global_checksum,
        }
    }
    pub fn cartridge_licensee_name(&self) -> String {
        if self.old_licensee_code == 0x33 {
            NewLicenseeCode::try_from(self.new_licensee_code as u8)
                .unwrap()
                .licensee_name()
                .to_string()
        } else {
            OldLicenseeCode::try_from(self.old_licensee_code as u8)
                .unwrap()
                .licensee_name()
                .to_string()
        }
    }

    pub fn title_name(&self) -> String {
        // String::from_utf8_lossy(&self.title).to_string()
        "".to_string()
    }

    // pub fn cartridge_type_name(&self){
    //     println!("Cartridge type: {}", self.cartridge_type);
    // }
}
