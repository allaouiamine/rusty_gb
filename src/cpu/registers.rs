use super::{
    instruction::RegisterType,
    util::{combine, ValueEnum},
};
use std::fmt::{Display, Result as FmtResult};

pub enum Flags {
    C = 4,
    H = 5,
    N = 6,
    Z = 7,
}

#[derive(Debug, Clone)]
pub struct FlagsRegister {
    register: u8,
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

    pub fn set_flag(&mut self, flag: Flags, flag_value: bool) {
        let mask: u8 = 1 << (flag as u8);

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

impl Display for CpuRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(
            f,
            "A: {:02X} F: {} BC: {} DE: {} HL: {}",
            self.a,
            self.f,
            self.get_register(RegisterType::BC),
            self.get_register(RegisterType::DE),
            self.get_register(RegisterType::HL),
        )
    }
}

impl CpuRegisters {
    pub fn new() -> Self {
        Self {
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
        }
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

    pub fn get_register(&self, register: RegisterType) -> ValueEnum {
        match register {
            RegisterType::A => ValueEnum::Data8(self.a),
            RegisterType::F => ValueEnum::Data8(self.f.register),
            RegisterType::B => ValueEnum::Data8(self.b),
            RegisterType::C => ValueEnum::Data8(self.c),
            RegisterType::D => ValueEnum::Data8(self.d),
            RegisterType::E => ValueEnum::Data8(self.e),
            RegisterType::H => ValueEnum::Data8(self.h),
            RegisterType::L => ValueEnum::Data8(self.l),

            // need to understand this a bit more
            RegisterType::AF => ValueEnum::Data16(combine(self.a, self.f.register)),
            RegisterType::BC => ValueEnum::Data16(combine(self.b, self.c)),
            RegisterType::DE => ValueEnum::Data16(combine(self.d, self.e)),
            RegisterType::HL => ValueEnum::Data16(combine(self.h, self.l)),

            RegisterType::PC => ValueEnum::Data16(self.pc),
            RegisterType::SP => ValueEnum::Data16(self.sp),
        }
    }

    pub fn set_register(&mut self, register: RegisterType, value: ValueEnum) {
        match value {
            ValueEnum::Data8(data) => match register {
                RegisterType::A => self.a = data,
                RegisterType::F => self.f.register = data,
                RegisterType::B => self.b = data,
                RegisterType::C => self.c = data,
                RegisterType::D => self.d = data,
                RegisterType::E => self.e = data,
                RegisterType::H => self.h = data,
                RegisterType::L => self.l = data,
                _ => {
                    panic!("cannot write Data8 to {}", register);
                }
            },
            ValueEnum::Data16(data) => {
                let lo = data as u8;
                let hi = (data >> 8) as u8;
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
                        self.pc = data;
                    }
                    RegisterType::SP => {
                        self.sp = data;
                    }
                    _ => {
                        panic!("cannot write Data16 to {}", register);
                    }
                }
            }
            ValueEnum::SignedData8(_) | ValueEnum::None => {
                panic!("set_register does not support signed data");
            }
        }
    }
}
