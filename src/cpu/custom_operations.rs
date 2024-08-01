use super::{
    execution_plan::CustomAction,
    registers::{CpuRegisters, Flags, REGISTERS_LOOKUP},
    AluOutput, CpuContext, RegisterType, ValueEnum,
};

pub trait CpuExtension {
    fn execute_custom(&mut self, fetched_data: ValueEnum) -> anyhow::Result<()>;
    fn process_cb(&mut self, prefix_cb: u8) -> anyhow::Result<()>;
}

impl<'a> CpuExtension for CpuContext<'a> {
    fn execute_custom(&mut self, fetched_data: ValueEnum) -> anyhow::Result<()> {
        match self.current_instruction.execution_plan.get_custom_action() {
            CustomAction::None => Ok(()),
            CustomAction::PrefixCB => self.process_cb(fetched_data.try_into()?),
            CustomAction::DI => {
                self.disable_master_interrupt();
                Ok(())
            }
            CustomAction::EI => {
                self.enable_master_interrupt_next_instruction();
                Ok(())
            }
            CustomAction::Halt => {
                self.halt();
                Ok(())
            }
            CustomAction::STOP => {
                panic!("STOP instruction called!");
            }
            CustomAction::PushPC => {
                self.push_pc();
                Ok(())
            }
            CustomAction::PopPC => {
                let address: u16 = self.stack_pop16().try_into()?;
                self.cpu_registers
                    .set_register_16(&RegisterType::PC, address);
                Ok(())
            }
            CustomAction::RETI => {
                let address: u16 = self.stack_pop16().try_into()?;
                self.cpu_registers
                    .set_register_16(&RegisterType::PC, address);
                self.enable_master_interrupt();
                Ok(())
            }
            CustomAction::RST => {
                let address = self.current_instruction.parameter.ok_or(anyhow::anyhow!(
                    "a parameter muse be provided for RST operations"
                ))? as u16;
                self.push_pc();
                self.cpu_registers
                    .set_register_16(&RegisterType::PC, address);
                Ok(())
            }
        }
    }
    fn process_cb(&mut self, prefix_cb: u8) -> anyhow::Result<()> {
        /*
         * The lower 3 bits 0, 1 and 2 of the opcode after prefix 0xCB are used to determine
         * the cpu register
         */
        let register_type = REGISTERS_LOOKUP[prefix_cb as usize & 0b111];

        self.emu_cycles(4); // Decoding the prefix CB takes 4 cycle

        let fetched_data: u8 = match &register_type {
            RegisterType::HL => {
                self.emu_cycles(4); // 16 bit register
                self.bus_read(self.cpu_registers.get_register_16(&RegisterType::HL))
            }
            other => self.cpu_registers.get_register(&other),
        };

        let alu_output = CbOperation::execute(prefix_cb, fetched_data, &self.cpu_registers)?;

        self.cpu_registers
            .set_flags(alu_output.z, alu_output.n, alu_output.h, alu_output.c);
        match register_type {
            RegisterType::HL => {
                self.emu_cycles(4); // 16 bit register
                self.bus_write(
                    self.cpu_registers.get_register_16(&RegisterType::HL),
                    alu_output.value.try_into()?,
                )
            }
            other => self
                .cpu_registers
                .set_register(&other, alu_output.value.try_into()?),
        }
        Ok(())
    }
}

pub struct CbOperation;

impl CbOperation {
    pub fn execute(
        prefix_cb: u8,
        fetched_data: u8,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        /* The high bits 6 and 7 determine the class of operation
        * 00: Rotate/Shift/SWAP: The next 3 bits: 3, 4 and 5 determine the operation
              in the order from 000 to 111: RLC, RRC, RL, RR, SLA, SRA, SWAP, SRL
        * 01: Test Bit: The next 3 bits determine the bit to test from bit0 to bit7
        * 10: Reset Bit: The next 3 bits determine the bit to reset from bit0 to bit7
        * 11: Set Bit: The next 3 bits determine the bit to set from bit0 to bit7
        */

        let operation_class = prefix_cb >> 6;
        let carry_flag = cpu_registers.f.get_flag_as_u8(Flags::C);
        Ok(match operation_class {
            0b00 => {
                // RLC, RRC, RL, RR, SLA, SRA, SWAP, SRL
                let operation = prefix_cb >> 3 & 0b111;
                let (value, carry) = match operation {
                    0b000 => {
                        // RLC rotate left: shift left then move uppermost bit to the rightmost bit
                        let rotated = (fetched_data << 1) | (fetched_data >> 7);
                        let carry = rotated & 0x01 == 0x1;
                        (rotated, carry)
                    }
                    0b001 => {
                        // RRC rotate right: shift right then move rightmost bit to the leftmost bit
                        let rotated = (fetched_data >> 1) | (fetched_data << 7);
                        let carry = rotated & 0x80 == 0x80;
                        (rotated, carry)
                    }
                    0b010 => {
                        // RL rotate left through carry: shift left then move carry to the rightmost bit
                        let carry = fetched_data & 0x80 == 0x80;
                        let rotated = (fetched_data << 1) | carry_flag;
                        (rotated, carry)
                    }
                    0b011 => {
                        // RR rotate right through carry: shift right then move carry to the leftmost bit
                        let carry = fetched_data & 0x01 == 0x01;
                        let rotated = (fetched_data >> 1) | (carry_flag << 7);
                        (rotated, carry)
                    }
                    0b100 => {
                        // SLA shift left: shift left then move 0 to the rightmost bit
                        let carry = fetched_data & 0x80 == 0x80;
                        let shifted = fetched_data << 1;
                        (shifted, carry)
                    }
                    0b101 => {
                        // SRA shift right: shift right then move the leftmost bit to the rightmost bit
                        let carry = fetched_data & 0x01 == 0x01;
                        let shifted = (fetched_data >> 1) | (fetched_data & 0x80); // keep the sign bit: BIT7
                        (shifted, carry)
                    }
                    0b110 => {
                        // SWAP swap nibbles: swap the upper and lower nibbles
                        let carry = false;
                        let swapped = (fetched_data >> 4) | (fetched_data << 4);
                        (swapped, carry)
                    }
                    0b111 => {
                        // SRL shift right: shift right then move 0 to the leftmost bit
                        let carry = fetched_data & 0x01 == 0x01;
                        let shifted = fetched_data >> 1;
                        (shifted, carry)
                    }
                    _ => panic!("operation can only be 000 to 111"),
                };
                AluOutput {
                    value: ValueEnum::Data8(value),
                    z: Some(value == 0),
                    n: Some(false),
                    h: Some(false),
                    c: Some(carry),
                    additional_cpu_cycles: 0,
                }
            }
            0b01 => {
                // Test Bit: The next 3 bits determine the bit to test from bit0 to bit7
                let bit_test_mask = 1 << ((prefix_cb >> 3) & 0b111);
                AluOutput {
                    value: ValueEnum::None,
                    z: Some((fetched_data & bit_test_mask) == bit_test_mask),
                    n: Some(false),
                    h: Some(true),
                    c: None,
                    additional_cpu_cycles: 0,
                }
            }
            0b10 => {
                // Reset Bit: The next 3 bits determine the bit to reset from bit0 to bit7
                let bit_reset_mask = !(1 << ((prefix_cb >> 3) & 0b111));
                let reset = fetched_data & bit_reset_mask;
                AluOutput {
                    value: ValueEnum::Data8(reset),
                    z: None,
                    n: None,
                    h: None,
                    c: None,
                    additional_cpu_cycles: 0,
                }
            }
            0b11 => {
                // Set Bit: The next 3 bits determine the bit to set from bit0 to bit7
                let bit_set_mask = 1 << ((prefix_cb >> 3) & 0b111);
                let set = fetched_data | bit_set_mask;
                AluOutput {
                    value: ValueEnum::Data8(set),
                    z: None,
                    n: None,
                    h: None,
                    c: None,
                    additional_cpu_cycles: 0,
                }
            }
            _ => panic!("operation_class can only be 00, 01, 10 or 11"),
        })
    }
}
