use std::fmt::Display;
use std::fmt::Result as FmtResult;

use super::instruction::InstructionType;
use super::instruction::Operand;
use super::CpuContext;

impl<'a> Display for CpuContext<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let mut pc = self.old_pc;

        let mut instruction_str = format!("{}", self.current_instruction);

        match self.current_instruction.operand_1 {
            Operand::None => {}
            Operand::Register(register) => {
                instruction_str.push_str(format!(" {}", register).as_str())
            }
            Operand::Indirect(register) => {
                instruction_str.push_str(format!(" ({})", register).as_str())
            }
            Operand::IndirectIncrementHL => instruction_str.push_str(format!(" (HL+)").as_str()),
            Operand::IndirectDecrementHL => instruction_str.push_str(format!(" (HL-)").as_str()),
            Operand::D8 | Operand::R8 => {
                pc += 1;
                instruction_str.push_str(format!(" ${:02X}", self.bus.bus_read(pc)).as_str())
            }
            Operand::A8Indirect => {
                pc += 1;
                if self.current_instruction.instruction_type == InstructionType::LDH {
                    instruction_str.push_str(format!(" ${:02X}", self.bus.bus_read(pc)).as_str())
                } else {
                    instruction_str.push_str(format!(" (${:02X})", self.bus.bus_read(pc)).as_str())
                }
            }
            Operand::A16 | Operand::D16 => {
                pc += 1;
                let next_pc = self.bus.bus_read(pc);
                pc += 1;
                let next_next_pc = self.bus.bus_read(pc);
                let value_u16 = (next_pc as u16) | ((next_next_pc as u16) << 8);
                instruction_str.push_str(format!(" ${:04X}", value_u16).as_str())
            }
            Operand::A16Indirect => {
                pc += 1;
                let next_pc = self.bus.bus_read(pc);
                pc += 1;
                let next_next_pc = self.bus.bus_read(pc);
                let value = (next_pc as u16) | ((next_next_pc as u16) << 8);
                instruction_str.push_str(format!(" (${:04X})", value).as_str())
            }
            Operand::SpPlusR8 => unimplemented!(),
        }

        match self.current_instruction.operand_2 {
            Operand::None => {}
            Operand::Register(register) => {
                instruction_str.push_str(format!(",{}", register).as_str())
            }
            Operand::Indirect(register) => {
                instruction_str.push_str(format!(",({})", register).as_str())
            }
            Operand::IndirectIncrementHL => instruction_str.push_str(format!(",(HL+)").as_str()),
            Operand::IndirectDecrementHL => instruction_str.push_str(format!(",(HL-)").as_str()),
            Operand::D8 | Operand::R8 => {
                pc += 1;
                instruction_str.push_str(format!(",${:02X}", self.bus.bus_read(pc)).as_str())
            }
            Operand::A8Indirect => {
                pc += 1;
                if self.current_instruction.instruction_type == InstructionType::LDH {
                    instruction_str.push_str(format!(",${:02X}", self.bus.bus_read(pc)).as_str())
                } else {
                    instruction_str.push_str(format!(",(${:02X})", self.bus.bus_read(pc)).as_str())
                }
            }
            Operand::A16 | Operand::D16 => {
                pc += 1;
                let next_pc = self.bus.bus_read(pc);
                pc += 1;
                let next_next_pc = self.bus.bus_read(pc);
                let value_u16 = (next_pc as u16) | ((next_next_pc as u16) << 8);
                instruction_str.push_str(format!(",${:04X}", value_u16).as_str())
            }
            Operand::A16Indirect => {
                pc += 1;
                let next_pc = self.bus.bus_read(pc);
                pc += 1;
                let next_next_pc = self.bus.bus_read(pc);
                let value_16 = (next_pc as u16) | ((next_next_pc as u16) << 8);
                instruction_str.push_str(format!(",(${:04X})", value_16).as_str())
            }
            Operand::SpPlusR8 => {
                pc += 1;
                let value_r8 = self.bus.bus_read(pc);
                instruction_str.push_str(format!(",SP+${:04X}", value_r8).as_str())
            }
        }

        write!(
            f,
            "{:08X} - {:04X}: {:12} ({:02X} {:02X} {:02X}) {}",
            self.ticks,
            self.old_pc,
            instruction_str,
            self.current_opcode,
            self.bus.bus_read(self.old_pc + 1),
            self.bus.bus_read(self.old_pc + 2),
            self.cpu_registers
        )
    }
}
