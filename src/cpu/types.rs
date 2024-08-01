use std::fmt::Error;

pub struct AluOutput {
    pub value: ValueEnum,
    pub z: Option<bool>,
    pub n: Option<bool>,
    pub h: Option<bool>,
    pub c: Option<bool>,
    pub additional_cpu_cycles: usize,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValueEnum {
    None,
    SignedData8(i8),
    Data8(u8),
    Data16(u16),
}

impl TryFrom<ValueEnum> for u16 {
    type Error = Error;

    fn try_from(value: ValueEnum) -> Result<Self, Self::Error> {
        match value {
            ValueEnum::SignedData8(_) | ValueEnum::None => Err(Error),
            ValueEnum::Data8(v) => Ok(v as u16),
            ValueEnum::Data16(v) => Ok(v as u16),
        }
    }
}

impl TryFrom<ValueEnum> for u8 {
    type Error = Error;

    fn try_from(value: ValueEnum) -> Result<Self, Self::Error> {
        match value {
            ValueEnum::SignedData8(_) | ValueEnum::None => Err(Error),
            ValueEnum::Data8(v) => Ok(v),
            ValueEnum::Data16(_) => Err(Error),
        }
    }
}

impl TryFrom<ValueEnum> for i8 {
    type Error = Error;

    fn try_from(value: ValueEnum) -> Result<Self, Self::Error> {
        match value {
            ValueEnum::SignedData8(v) => Ok(v),
            ValueEnum::Data8(v) => Ok(v as i8),
            ValueEnum::Data16(_) | ValueEnum::None => Err(Error),
        }
    }
}

pub enum Bits {
    C = 4,
    Z = 7,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RegisterType {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(PartialEq, Debug)]
pub enum InterruptType {
    VBLANK = 1,
    LCDStat = 2,
    TIMER = 4,
    SERIAL = 8,
    JOYPAD = 16,
}

impl From<u8> for InterruptType {
    fn from(value: u8) -> Self {
        if value & 0x1 != 0 {
            Self::VBLANK
        } else if (value >> 1) & 0x1 != 0 {
            Self::LCDStat
        } else if (value >> 2) & 0x1 != 0 {
            Self::TIMER
        } else if (value >> 3) & 0x1 != 0 {
            Self::SERIAL
        } else if (value >> 4) & 0x1 != 0 {
            Self::JOYPAD
        } else {
            unimplemented!();
        }
    }
}
