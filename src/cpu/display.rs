use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use crate::cpu::execution_plan::FetchAction;
use crate::cpu::execution_plan::StoreAction;
use crate::cpu::instruction::ConditionType;

use super::instruction::Instruction;
use super::registers::CpuRegisters;
use super::types::RegisterType;
use super::types::ValueEnum;
use super::CpuContext;

impl Display for ValueEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        match self {
            ValueEnum::SignedData8(_) | ValueEnum::None => panic!("Cannot display SignedData8"),
            ValueEnum::Data8(value) => write!(f, "{:02X}", value),
            ValueEnum::Data16(value) => write!(f, "{:04X}", value),
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

impl Display for CpuRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(
            f,
            "A: {:02X} F: {} BC: {:04X} DE: {:04X} HL: {:04X} SP: {:04X}",
            self.a,
            self.f,
            self.get_register_16(&RegisterType::BC),
            self.get_register_16(&RegisterType::DE),
            self.get_register_16(&RegisterType::HL),
            self.sp
        )
    }
}

impl<'a> Display for Instruction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.instruction_type)
    }
}

impl Display for ConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let condition_str = match *self {
            Self::None => "",
            Self::NZ => "NZ",
            Self::Z => "Z",
            Self::NC => "NC",
            Self::C => "C",
        };
        write!(f, "{}", condition_str)
    }
}

impl<'a> Display for CpuContext<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let mut instruction_str = format!("{}", self.current_instruction);
        let bus = self.bus.lock().unwrap();

        let operand_1 = match self.current_instruction.execution_plan.get_fetch_action() {
            FetchAction::None | FetchAction::FetchStack => None,
            FetchAction::FetchData | FetchAction::FetchSignedData => {
                Some(format!("${:02X}", bus.bus_read(self.old_pc + 1)))
            }
            FetchAction::FetchData16Bits => {
                let lo = bus.bus_read(self.old_pc + 1);
                let hi = bus.bus_read(self.old_pc + 2);
                Some(format!("${:04X}", (lo as u16) | ((hi as u16) << 8)))
            }
            FetchAction::FetchAddress => {
                let lo = bus.bus_read(self.old_pc + 1);
                let hi = bus.bus_read(self.old_pc + 2);
                Some(format!("$({:04X})", (lo as u16) | ((hi as u16) << 8)))
            }
            FetchAction::FetchAddressZeroPage => {
                let lo = bus.bus_read(self.old_pc + 1);
                Some(format!("$FF({:02X})", lo))
            }
            FetchAction::FetchRegister(register_type)
            | FetchAction::FetchRegister16Bits(register_type) => Some(format!("{}", register_type)),
            FetchAction::FetchRegister16BitsWithOffset(register_type) => {
                if register_type == &RegisterType::PC {
                    Some(format!("${:02X}", bus.bus_read(self.old_pc + 1)))
                } else {
                    Some(format!(
                        "{}+${:02X}",
                        register_type,
                        bus.bus_read(self.old_pc + 1)
                    ))
                }
            }
            FetchAction::FetchIndirect(register_type) => Some(format!("({})", register_type)),
            FetchAction::FetchIndirectZeroPage(register_type) => {
                Some(format!("({})", register_type))
            }
            FetchAction::FetchIndirectAndIncrement(register_type) => {
                Some(format!("({}+)", register_type))
            }
            FetchAction::FetchIndirectAndDecrement(register_type) => {
                Some(format!("({}-)", register_type))
            }
        };

        let operand_2 = match self.current_instruction.execution_plan.get_store_action() {
            StoreAction::None | StoreAction::StoreStack => None,
            StoreAction::StoreRegister(register_type)
            | StoreAction::StoreRegister16Bits(register_type) => {
                if register_type == &RegisterType::PC {
                    None
                } else {
                    Some(format!("{}", register_type))
                }
            }
            StoreAction::StoreIndirect(register_type) => Some(format!("({})", register_type)),
            StoreAction::StoreIndirectZeroPage(register_type) => {
                Some(format!("({})", register_type))
            }
            StoreAction::StoreIndirectAndIncrement(register_type) => {
                Some(format!("({}+)", register_type))
            }
            StoreAction::StoreIndirectAndDecrement(register_type) => {
                Some(format!("({}-)", register_type))
            }
            StoreAction::StoreAddress | StoreAction::StoreAddress16Bits => {
                let lo = bus.bus_read(self.old_pc + 1);
                let hi = bus.bus_read(self.old_pc + 2);
                Some(format!("$({:04X})", (lo as u16) | ((hi as u16) << 8)))
            }
            StoreAction::StoreAddressZeroPage => {
                let lo = bus.bus_read(self.old_pc + 1);
                Some(format!("($FF{:02X})", lo))
            }
        };
        if let Some(operand_2) = operand_2 {
            //instruction_str = format!("{} {}", instruction_str, operand_2);
        }

        if self.current_instruction.condition != ConditionType::None {
            //instruction_str = format!("{} {}", instruction_str, self.current_instruction.condition);
        }
        if let Some(operand_1) = operand_1 {
            // instruction_str = format!("{} {}", instruction_str, operand_1);
        }
        write!(
            f,
            "{:08X} - {:04X}: {:12} ({:02X} {:02X} {:02X}) {}",
            self.ticks / 4,
            self.old_pc,
            instruction_str,
            self.current_opcode,
            bus.bus_read(self.old_pc + 1),
            bus.bus_read(self.old_pc + 2),
            self.cpu_registers
        )
    }
}
