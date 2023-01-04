use core::panic;
use std::fmt::{Display, Error, Result as FmtResult};

use super::RegisterType;

#[derive(Debug)]
pub enum DestinationEnum {
    None,
    Register(RegisterType),
    Address(u16),
}

impl Default for DestinationEnum {
    fn default() -> Self {
        DestinationEnum::None
    }
}

#[derive(Debug)]
pub struct FetchedData {
    pub source: ValueEnum,
    pub destination: DestinationEnum,
}

impl Default for FetchedData {
    fn default() -> Self {
        Self {
            source: Default::default(),
            destination: Default::default(),
        }
    }
}

impl FetchedData {
    pub fn with_source(source: ValueEnum) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }

    pub fn with_destination(destination: DestinationEnum) -> Self {
        Self {
            destination,
            ..Default::default()
        }
    }

    pub fn new(source: ValueEnum, destination: DestinationEnum) -> Self {
        Self {
            source,
            destination,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ValueEnum {
    None,
    SignedData8(i8),
    Data8(u8),
    Data16(u16),
}

impl Default for ValueEnum {
    fn default() -> Self {
        ValueEnum::None
    }
}

impl Display for ValueEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        match self {
            ValueEnum::SignedData8(_) | ValueEnum::None => panic!("Cannot display SignedData8"),
            ValueEnum::Data8(value) => write!(f, "{:02X}", value),
            ValueEnum::Data16(value) => write!(f, "{:04X}", value),
        }
    }
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

pub enum Bits {
    C = 4,
    Z = 7,
}

pub fn is_bit_set(register: u8, bits: Bits) -> bool {
    let mask: u8 = 1 << (bits as u8);
    register & mask == mask
}

pub fn combine(reg_left: u8, reg_right: u8) -> u16 {
    let hi = (reg_left as u16) << 8;
    let lo = reg_right as u16;
    hi | lo
}
