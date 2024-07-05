use super::{util::combine, RegisterType};
use std::fmt::{Display, Result as FmtResult};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Flags {
    C = 4,
    H = 5,
    N = 6,
    Z = 7,
}

#[derive(Debug, Clone)]
pub struct FlagsRegister {
    pub register: u8,
}

impl FlagsRegister {
    pub fn new() -> Self {
        Self {
            register: 0b10110000,
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        let mask = 1 << (flag as u8);
        self.register & mask == mask
    }

    pub fn get_flag_as_u8(&self, flag: Flags) -> u8 {
        if self.get_flag(flag) {
            1
        } else {
            0
        }
    }

    pub fn set_flag(&mut self, flag: Flags, flag_value: bool) {
        let mask: u8 = 1 << (flag.clone() as u8);

        if flag_value {
            self.register |= mask;
        } else {
            self.register &= !mask;
        }
    }
}

impl Display for FlagsRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let z = match self.get_flag(Flags::Z) {
            true => "Z",
            false => "-",
        };

        let n = match self.get_flag(Flags::N) {
            true => "N",
            false => "-",
        };
        let h = match self.get_flag(Flags::H) {
            true => "H",
            false => "-",
        };
        let c = match self.get_flag(Flags::C) {
            true => "C",
            false => "-",
        };
        write!(f, "{}{}{}{}", z, n, h, c)
    }
}

#[derive(Debug, Clone)]
pub struct CpuRegisters {
    pub a: u8,
    pub f: FlagsRegister,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16, // program counter
    pub sp: u16, // stack pointer
}

impl CpuRegisters {
    pub fn new() -> Self {
        let mut regs = Self {
            a: 0x01,
            f: FlagsRegister::new(),
            b: 0,
            c: 0x13,
            d: 0,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x100,
            sp: 0xFFFE,
        };
        regs.f.set_flag(Flags::Z, false);
        regs.f.set_flag(Flags::N, false);
        regs.f.set_flag(Flags::H, false);
        regs.f.set_flag(Flags::C, false);
        regs
    }

    pub fn set_flags(
        &mut self,
        z: Option<bool>,
        n: Option<bool>,
        h: Option<bool>,
        c: Option<bool>,
    ) {
        if let Some(value) = z {
            self.f.set_flag(Flags::Z, value);
        }

        if let Some(value) = n {
            self.f.set_flag(Flags::N, value)
        }

        if let Some(value) = h {
            self.f.set_flag(Flags::H, value)
        }

        if let Some(value) = c {
            self.f.set_flag(Flags::C, value)
        }
    }

    pub fn get_register(&self, register: &RegisterType) -> u8 {
        match register {
            RegisterType::A => self.a,
            RegisterType::F => self.f.register,
            RegisterType::B => self.b,
            RegisterType::C => self.c,
            RegisterType::D => self.d,
            RegisterType::E => self.e,
            RegisterType::H => self.h,
            RegisterType::L => self.l,
            other => panic!("Invalid 8 bits register type: {:?}", other),
        }
    }

    pub fn get_register_16(&self, register: &RegisterType) -> u16 {
        match register {
            RegisterType::AF => combine(self.a, self.f.register),
            RegisterType::BC => combine(self.b, self.c),
            RegisterType::DE => combine(self.d, self.e),
            RegisterType::HL => combine(self.h, self.l),
            RegisterType::SP => self.sp,
            RegisterType::PC => self.pc,
            other => panic!("Invalid 16 bits register type: {:?}", other),
        }
    }

    pub fn set_register(&mut self, register: &RegisterType, value: u8) {
        match register {
            RegisterType::A => self.a = value,
            RegisterType::F => self.f.register = value,
            RegisterType::B => self.b = value,
            RegisterType::C => self.c = value,
            RegisterType::D => self.d = value,
            RegisterType::E => self.e = value,
            RegisterType::H => self.h = value,
            RegisterType::L => self.l = value,
            other => panic!("Invalid 8 bits register type: {:?}", other),
        }
    }

    pub fn set_register_16(&mut self, register: &RegisterType, value: u16) {
        let lo = value as u8;
        let hi = (value >> 8) as u8;
        match register {
            RegisterType::AF => {
                self.a = hi;
                self.f.register = lo & 0xF0;
            }
            RegisterType::BC => {
                self.b = hi;
                self.c = lo;
            }
            RegisterType::DE => {
                self.d = hi;
                self.e = lo;
            }
            RegisterType::HL => {
                self.h = hi;
                self.l = lo;
            }
            RegisterType::PC => {
                self.pc = value;
            }
            RegisterType::SP => {
                self.sp = value;
            }
            other => panic!("Invalid 16 bits register type: {:?}", other),
        }
    }
}
