use std::fmt::Display;
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone)]
pub struct Instruction<'a> {
    pub description: &'a str,
    pub instruction_type: InstructionType,
    pub operand_1: Operand,
    pub operand_2: Operand,
    pub condition: ConditionType,
    pub parameters: Option<u8>,
}

impl<'a> Display for Instruction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.instruction_type)
    }
}

impl<'i> Default for Instruction<'i> {
    fn default() -> Self {
        Self {
            description: "NONE",
            instruction_type: Default::default(),
            operand_1: Default::default(),
            operand_2: Default::default(),
            condition: Default::default(),
            parameters: Default::default(),
        }
    }
}

impl<'i> Instruction<'i> {
    pub fn instruction_no_operands(
        description: &'i str,
        instruction_type: InstructionType,
    ) -> Self {
        Self {
            description,
            instruction_type,
            ..Default::default()
        }
    }
    pub fn instruction_no_operands_with_condition(
        description: &'i str,
        instruction_type: InstructionType,
        condition: ConditionType,
    ) -> Self {
        Self {
            description,
            instruction_type,
            condition,
            ..Default::default()
        }
    }
    pub fn instruction_1_operand(
        description: &'i str,
        instruction_type: InstructionType,
        operand_1: Operand,
    ) -> Self {
        Self {
            description,
            instruction_type,
            operand_1,
            ..Default::default()
        }
    }

    pub fn instruction_1_operand_with_condition(
        description: &'i str,
        instruction_type: InstructionType,
        operand_1: Operand,
        condition: ConditionType,
    ) -> Self {
        Self {
            description,
            instruction_type,
            operand_1,
            condition,
            ..Default::default()
        }
    }
    pub fn instruction_2_operands(
        description: &'i str,
        instruction_type: InstructionType,
        operand_1: Operand,
        operand_2: Operand,
    ) -> Self {
        Self {
            description,
            instruction_type,
            operand_1,
            operand_2,
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone)]
pub enum Operand {
    None,
    Register(RegisterType),
    Indirect(RegisterType), // (HL)
    IndirectIncrementHL,    // (HL+)
    IndirectDecrementHL,    // (HL+)
    A8Indirect,
    A16,
    A16Indirect,
    D8,
    D16,
    R8,
    SpPlusR8,
}

impl Default for Operand {
    fn default() -> Self {
        Operand::None
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ConditionType {
    None,
    NZ,
    Z,
    NC,
    C,
}

impl Default for ConditionType {
    fn default() -> Self {
        ConditionType::None
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum InstructionType {
    NONE,
    NOP,
    LD,
    INC,
    DEC,
    RLCA,
    ADD,
    RRCA,
    STOP,
    RLA,
    JR,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    POP,
    JP,
    PUSH,
    RET,
    CB,
    CALL,
    RETI,
    LDH,
    JPHL,
    DI,
    EI,
    RST,
    ERR,
    //CB instructions...
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
}

impl Default for InstructionType {
    fn default() -> Self {
        InstructionType::NONE
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let instruction_str = match self {
            InstructionType::NONE => "NONE",
            InstructionType::NOP => "NOP",
            InstructionType::LD => "LD",
            InstructionType::INC => "INC",
            InstructionType::DEC => "DEC",
            InstructionType::RLCA => "RLCA",
            InstructionType::ADD => "ADD",
            InstructionType::RRCA => "RRCA",
            InstructionType::STOP => "STOP",
            InstructionType::RLA => "RLA",
            InstructionType::JR => "JR",
            InstructionType::RRA => "RRA",
            InstructionType::DAA => "DAA",
            InstructionType::CPL => "CPL",
            InstructionType::SCF => "SCF",
            InstructionType::CCF => "CCF",
            InstructionType::HALT => "HALT",
            InstructionType::ADC => "ADC",
            InstructionType::SUB => "SUB",
            InstructionType::SBC => "SBC",
            InstructionType::AND => "AND",
            InstructionType::XOR => "XOR",
            InstructionType::OR => "OR",
            InstructionType::CP => "CP",
            InstructionType::POP => "POP",
            InstructionType::JP => "JP",
            InstructionType::PUSH => "PUSH",
            InstructionType::RET => "RET",
            InstructionType::CB => "CB",
            InstructionType::CALL => "CALL",
            InstructionType::RETI => "RETI",
            InstructionType::LDH => "LDH",
            InstructionType::JPHL => "JPHL",
            InstructionType::DI => "DI",
            InstructionType::EI => "EI",
            InstructionType::RST => "RST",
            InstructionType::ERR => "ERR",
            InstructionType::RLC => "RLC",
            InstructionType::RRC => "RRC",
            InstructionType::RL => "RL",
            InstructionType::RR => "RR",
            InstructionType::SLA => "SLA",
            InstructionType::SRA => "SRA",
            InstructionType::SWAP => "SWAP",
            InstructionType::SRL => "SRL",
            InstructionType::BIT => "BIT",
            InstructionType::RES => "RES",
            InstructionType::SET => "SET",
        };
        write!(f, "{}", instruction_str)
    }
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

impl RegisterType {
    pub fn is_16bit(&self) -> bool {
        match self {
            RegisterType::AF
            | RegisterType::BC
            | RegisterType::DE
            | RegisterType::HL
            | RegisterType::SP
            | RegisterType::PC => true,
            _ => false,
        }
    }
}

impl Display for RegisterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let register_str = match *self {
            Self::A => "A",
            Self::F => "F",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::H => "H",
            Self::L => "L",
            Self::AF => "AF",
            Self::BC => "BC",
            Self::DE => "DE",
            Self::HL => "HL",
            Self::SP => "SP",
            Self::PC => "PC",
        };
        write!(f, "{}", register_str)
    }
}
