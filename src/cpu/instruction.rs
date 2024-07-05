use std::fmt::Display;
use std::fmt::{Formatter, Result as FmtResult};

use super::execution_plan::ExecutionPlan;

#[derive(Debug)]
pub struct Instruction<'a> {
    pub description: &'a str,
    pub instruction_type: InstructionType,
    pub execution_plan: ExecutionPlan,
    pub condition: ConditionType,
    pub parameter: Option<u8>,
}


impl<'i> Default for Instruction<'i> {
    fn default() -> Self {
        Self {
            description: "NONE",
            instruction_type: Default::default(),
            condition: Default::default(),
            parameter: Default::default(),
            execution_plan: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
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

