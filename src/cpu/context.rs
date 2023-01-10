use core::panic;

use crate::bus::Bus;
use crate::cpu::util::add_relative;

use super::instruction::ConditionType;
use super::instruction::InstructionType;
use super::instruction::Operand;
use super::instruction::RegisterType;
use super::registers::CpuRegisters;
use super::registers::Flags;

use super::util::check_carry_relative;
use super::util::check_half_carry_relative;
use super::util::DestinationEnum;
use super::util::FetchedData;
use super::util::ValueEnum;
use super::{instruction::Instruction, instruction_set::InstructionSet};

pub struct CpuContext<'a> {
    pub bus: Bus<'a>,
    instruction_set: &'a InstructionSet,
    pub cpu_registers: CpuRegisters,
    pub current_instruction: &'a Instruction<'a>,
    fetched_data: FetchedData,
    pub old_registers: CpuRegisters,
    pub current_opcode: u8,
    halted: bool,
    pub ticks: usize,
    interrupt_master_enabled: bool,
    enabling_ime: bool,
}

impl<'a> CpuContext<'a> {
    pub fn new(rom_file: &'a str, instruction_set: &'a InstructionSet) -> Self {
        let bus = Bus::new(rom_file);
        Self {
            bus,
            instruction_set,
            cpu_registers: CpuRegisters::new(),
            current_instruction: instruction_set.get_instruction_by_opcode(0x00),
            fetched_data: FetchedData {
                source: ValueEnum::None,
                destination: DestinationEnum::None,
            },
            old_registers: CpuRegisters::new(),
            current_opcode: 0,
            halted: false,
            // stepping: false,
            ticks: 0,
            interrupt_master_enabled: false,
            enabling_ime: false,
        }
    }

    fn get_interrupt_enable_register(&self) -> u8 {
        self.bus.bus_read(0xFFFF)
    }

    fn get_interrupt_flags_register(&self) -> u8 {
        self.bus.bus_read(0xFF0F)
    }

    fn set_interrupt_flags_register(&mut self, value: u8) {
        self.bus.bus_write8(0xFF0F, value);
    }

    fn request_interrupt(&mut self, interrupt_type: InterruptType) {
        let interrupt_flags = self.get_interrupt_flags_register();
        self.set_interrupt_flags_register(interrupt_flags | (interrupt_type as u8));
    }

    fn interrupt_handle(&mut self, address: u16) {
        // Do not count the CPU ticks during interrupts!
        // self.stack_push16(self.cpu_registers.pc);
        self.cpu_registers.sp -= 1;
        self.bus
            .bus_write8(self.cpu_registers.sp, (self.cpu_registers.pc >> 8) as u8);
        self.cpu_registers.sp -= 1;
        self.bus
            .bus_write8(self.cpu_registers.sp, self.cpu_registers.pc as u8);
        self.cpu_registers.pc = address;
    }

    fn cpu_handle_interrupts(&mut self) {
        let interrupt_flags = self.get_interrupt_flags_register();

        let interrupt_enable = self.get_interrupt_enable_register();

        let allowed_interrupts = interrupt_flags & interrupt_enable;

        if allowed_interrupts == 0 {
            return;
        }

        let interrupt_type = InterruptType::from(allowed_interrupts);

        // if multiple interrupts requested, chose the highest priority, run it and leave the others
        let address: u16 = match interrupt_type {
            InterruptType::VBLANK => 0x40,
            InterruptType::LCDStat => 0x48,
            InterruptType::TIMER => 0x50,
            InterruptType::SERIAL => 0x58,
            InterruptType::JOYPAD => 0x60,
        };

        self.interrupt_handle(address);

        // reset in interrupt_flags bit
        self.set_interrupt_flags_register(interrupt_flags & !(interrupt_type as u8));
        self.halted = false;
        self.interrupt_master_enabled = false;
    }

    pub fn get_register(&self, register_type: RegisterType) -> ValueEnum {
        self.cpu_registers.get_register(register_type)
    }

    pub fn set_register(&mut self, register_type: RegisterType, value: ValueEnum) {
        self.cpu_registers.set_register(register_type, value);
    }

    pub fn bus_read(&mut self, address: u16) -> u8 {
        self.emu_cycles(1);
        self.bus.bus_read(address)
    }

    pub fn bus_read16(&mut self, address: u16) -> u16 {
        self.emu_cycles(2);
        self.bus.bus_read16(address)
    }

    pub fn bus_write(&mut self, address: u16, value: ValueEnum) {
        match value {
            ValueEnum::Data8(data) => {
                self.bus.bus_write8(address, data);
                self.emu_cycles(1);
            }
            ValueEnum::Data16(data) => {
                self.bus.bus_write16(address, data);
                self.emu_cycles(2);
            }
            ValueEnum::SignedData8(_) | ValueEnum::None => {
                panic!("cannot bus_write signed data!");
            }
        }
    }

    pub fn stack_push(&mut self, data: u8) {
        self.cpu_registers.sp -= 1;
        self.bus_write(self.cpu_registers.sp, ValueEnum::Data8(data));
    }

    pub fn stack_push16(&mut self, data: u16) {
        self.stack_push((data >> 8) as u8);
        self.stack_push(data as u8);
    }

    pub fn stack_pop(&mut self) -> u8 {
        let data = self.bus_read(self.cpu_registers.sp);
        self.cpu_registers.sp += 1;
        data
    }

    pub fn stack_pop16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        lo | (hi << 8)
    }

    pub fn fetch_instruction(&mut self) {
        let pc: u16 = self.cpu_registers.pc;
        self.current_opcode = self.bus_read(pc);

        self.current_instruction = self
            .instruction_set
            .get_instruction_by_opcode(self.current_opcode);

        self.cpu_registers.pc += 1;
    }

    fn fetch_data(&mut self) {
        match self.current_instruction.instruction_type {
            InstructionType::NOP
            | InstructionType::HALT
            | InstructionType::DI
            | InstructionType::EI
            | InstructionType::RET
            | InstructionType::RETI
            | InstructionType::RST
            | InstructionType::RRA
            | InstructionType::CPL
            | InstructionType::SCF
            | InstructionType::CCF
            | InstructionType::RLCA
            | InstructionType::DAA => {}
            InstructionType::JP
            | InstructionType::JR
            | InstructionType::CALL
            | InstructionType::PUSH
            | InstructionType::OR
            | InstructionType::XOR
            | InstructionType::AND
            | InstructionType::CP
            | InstructionType::SUB
            | InstructionType::CB => {
                self.fetch_data_1_operand();
            }
            InstructionType::INC | InstructionType::DEC | InstructionType::POP => {
                self.fetch_data_1_operand_mut();
            }
            InstructionType::LD
            | InstructionType::LDH
            | InstructionType::ADD
            | InstructionType::ADC
            | InstructionType::SBC => {
                self.fetch_data_2_operands();
            }
            other => {
                panic!(
                    "unimplemented instruction: {}, opcode: {:02X}",
                    other, self.current_opcode
                );
            }
        }
    }

    pub fn cpu_step(&mut self) -> bool {
        self.old_registers = self.cpu_registers.clone();
        if !self.halted {
            self.fetch_instruction();
            self.fetch_data();
            println!("{}", self);
            self.bus.dbg_update();
            self.bus.dbg_print();
            self.execute();
        } else {
            self.emu_cycles(1);
            if self.get_interrupt_flags_register() != 0 {
                self.halted = false;
            }
        }

        if self.interrupt_master_enabled {
            self.cpu_handle_interrupts();
            self.enabling_ime = false;
        }

        if self.enabling_ime {
            self.interrupt_master_enabled = true;
        }
        true
    }
    pub fn get_next_pc_value(&mut self) -> u8 {
        let value = self.bus_read(self.cpu_registers.pc);
        self.cpu_registers.pc += 1;
        value
    }
    fn emu_cycles(&mut self, ticks: usize) {
        for _ in 0..ticks {
            for _ in 0..4 {
                self.ticks += 1;

                if let Some(interrupt) = self.bus.io.timer.timer_tick() {
                    self.request_interrupt(interrupt);
                }
            }
        }
    }

    pub fn increment_and_return_hl(&mut self) -> u16 {
        let old_hl_value: u16 = self.get_register(RegisterType::HL).try_into().unwrap();
        self.set_register(RegisterType::HL, ValueEnum::Data16(old_hl_value + 1));
        old_hl_value
    }

    pub fn decrement_and_return_hl(&mut self) -> u16 {
        let old_hl_value: u16 = self.get_register(RegisterType::HL).try_into().unwrap();
        self.set_register(RegisterType::HL, ValueEnum::Data16(old_hl_value - 1));
        old_hl_value
    }

    pub fn fetch_source_data(&mut self, operand: Operand) -> ValueEnum {
        match operand {
            Operand::None => {
                unimplemented!();
            }
            Operand::Register(register_type) => self.cpu_registers.get_register(register_type),
            Operand::Indirect(register_type) => {
                let register_value: u16 = self
                    .cpu_registers
                    .get_register(register_type)
                    .try_into()
                    .unwrap();
                let value;
                if register_type == RegisterType::C {
                    value = self.bus_read(register_value | 0xFF00);
                } else {
                    value = self.bus_read(register_value);
                }
                ValueEnum::Data8(value)
            }
            Operand::IndirectIncrementHL => {
                if self.current_instruction.instruction_type != InstructionType::LD {
                    unimplemented!();
                }
                let hl = self.increment_and_return_hl();
                let value = self.bus_read(hl);
                ValueEnum::Data8(value)
            }
            Operand::IndirectDecrementHL => {
                if self.current_instruction.instruction_type != InstructionType::LD {
                    unimplemented!();
                }
                let hl = self.decrement_and_return_hl();
                let value = self.bus_read(hl);
                ValueEnum::Data8(value)
            }
            Operand::A8Indirect => {
                if self.current_instruction.instruction_type != InstructionType::LDH {
                    unimplemented!();
                }
                let address = (self.get_next_pc_value() as u16) | 0xFF00;
                let value = self.bus_read(address);
                ValueEnum::Data8(value)
            }
            Operand::A16Indirect => {
                if self.current_instruction.instruction_type != InstructionType::LD {
                    unimplemented!();
                }
                let lo = self.get_next_pc_value() as u16;
                let hi = self.get_next_pc_value() as u16;
                let address = (hi << 8) | lo;
                let value = self.bus_read(address);
                ValueEnum::Data8(value)
            }
            Operand::A16 => {
                if self.current_instruction.instruction_type != InstructionType::JP
                    && self.current_instruction.instruction_type != InstructionType::CALL
                {
                    unimplemented!();
                }
                let lo = self.get_next_pc_value() as u16;
                let hi = self.get_next_pc_value() as u16;
                ValueEnum::Data16((hi << 8) | lo)
            }
            Operand::D8 => ValueEnum::Data8(self.get_next_pc_value()),
            Operand::D16 => {
                let lo = self.get_next_pc_value() as u16;
                let hi = self.get_next_pc_value() as u16;
                ValueEnum::Data16((hi << 8) | lo)
            }
            Operand::R8 => {
                // only used in 0xE8 ADD SP,r8
                ValueEnum::SignedData8(self.get_next_pc_value() as i8)
            }
            Operand::SpPlusR8 => {
                if self.current_opcode != 0xF8 {
                    unimplemented!();
                }
                // only used in 0xF9 LD HL,SP+r8 (should set the H and C flags)
                let signed_data = self.get_next_pc_value() as i8;
                let sp_value = self.cpu_registers.sp;

                let value = add_relative(sp_value, signed_data);

                let h = check_half_carry_relative(sp_value, signed_data);
                let c = check_carry_relative(sp_value, signed_data);

                self.cpu_registers
                    .set_flags(Some(false), Some(false), Some(h), Some(c));
                ValueEnum::Data16(value)
            }
        }
    }

    fn fetch_destination_data(&mut self, operand: Operand) -> DestinationEnum {
        match operand {
            Operand::Register(register_type) => DestinationEnum::Register(register_type),
            Operand::Indirect(register_type) => {
                let register_value = self
                    .cpu_registers
                    .get_register(register_type)
                    .try_into()
                    .unwrap();
                if register_type == RegisterType::C {
                    DestinationEnum::Address(register_value | 0xFF00)
                } else {
                    DestinationEnum::Address(register_value)
                }
            }
            Operand::IndirectIncrementHL => {
                if self.current_instruction.instruction_type != InstructionType::LD {
                    unimplemented!();
                }
                DestinationEnum::Address(self.increment_and_return_hl())
            }
            Operand::IndirectDecrementHL => {
                if self.current_instruction.instruction_type != InstructionType::LD {
                    unimplemented!();
                }
                DestinationEnum::Address(self.decrement_and_return_hl())
            }
            Operand::A8Indirect => {
                if self.current_instruction.instruction_type != InstructionType::LDH {
                    unimplemented!();
                }
                DestinationEnum::Address((self.get_next_pc_value() as u16) | 0xFF00)
            }
            Operand::A16Indirect => {
                if self.current_instruction.instruction_type != InstructionType::LD {
                    unimplemented!();
                }
                let lo = self.get_next_pc_value() as u16;
                let hi = self.get_next_pc_value() as u16;
                DestinationEnum::Address((hi << 8) | lo)
            }

            Operand::None
            | Operand::A16
            | Operand::D8
            | Operand::D16
            | Operand::R8
            | Operand::SpPlusR8 => {
                unimplemented!()
            }
        }
    }

    pub fn fetch_data_1_operand(&mut self) {
        self.fetched_data =
            FetchedData::with_source(self.fetch_source_data(self.current_instruction.operand_1));
    }

    pub fn fetch_data_2_operands(&mut self) {
        let current_instruction = self.current_instruction;

        let source = self.fetch_source_data(current_instruction.operand_2);
        let destination = self.fetch_destination_data(current_instruction.operand_1);
        self.fetched_data = FetchedData::new(source, destination);
    }

    fn fetch_data_1_operand_mut(&mut self) {
        self.fetched_data = FetchedData::with_destination(
            self.fetch_destination_data(self.current_instruction.operand_1),
        );
    }

    fn check_condition(&mut self) -> bool {
        let z = self.cpu_registers.f.get_flag(Flags::Z);
        let c = self.cpu_registers.f.get_flag(Flags::C);

        match self.current_instruction.condition {
            ConditionType::C => c,
            ConditionType::Z => z,
            ConditionType::NC => !c,
            ConditionType::NZ => !z,
            ConditionType::None => true,
        }
    }

    pub fn execute(&mut self) {
        match self.current_instruction.instruction_type {
            InstructionType::NONE => self.process_none(),
            InstructionType::NOP => self.process_nop(),

            //InstructionType::LD | InstructionType::LDH => self.process_ld(),
            InstructionType::LD => self.process_ld(),
            InstructionType::LDH => self.process_ldh(),
            InstructionType::ADD => self.process_add(),
            InstructionType::ADC => self.process_adc(),
            InstructionType::SUB => self.process_sub(),
            InstructionType::SBC => self.process_sbc(),
            InstructionType::RST => self.process_rst(),
            InstructionType::JP => self.process_jp(),
            InstructionType::JR => self.process_jr(),
            InstructionType::CALL => self.process_call(),
            InstructionType::RET => self.process_ret(),
            InstructionType::RETI => self.process_reti(),
            InstructionType::INC => self.process_inc(),
            InstructionType::DEC => self.process_dec(),
            InstructionType::PUSH => self.process_push(),
            InstructionType::POP => self.process_pop(),
            InstructionType::OR => self.process_or(),
            InstructionType::XOR => self.process_xor(),
            InstructionType::AND => self.process_and(),
            InstructionType::CP => self.process_cp(),

            InstructionType::CB => self.process_cb(),
            InstructionType::RRA => self.process_rra(),
            InstructionType::CPL => self.process_cpl(),
            InstructionType::SCF => self.process_scf(),
            InstructionType::CCF => self.process_ccf(),
            InstructionType::RLCA => self.process_rlca(),

            InstructionType::DAA => self.process_daa(),

            InstructionType::DI => {
                self.interrupt_master_enabled = false;
            }
            InstructionType::EI => {
                self.enabling_ime = true;
            }
            InstructionType::HALT => {
                self.halted = true;
            }
            _ => {
                panic!("NOT YET IMPLEMENTED!!");
            }
        }
    }

    fn process_none(&mut self) {
        panic!("Unknown instruction: {:#04X}", self.current_opcode);
    }

    fn process_nop(&mut self) {}

    fn process_ld(&mut self) {
        /* Should set Flags on 0xF8  0 0 H C
        It is already done in fetch_data_2_operands
        */

        match self.fetched_data.destination {
            DestinationEnum::Register(register_type) => {
                self.set_register(register_type, self.fetched_data.source);
            }
            DestinationEnum::Address(address) => {
                self.bus_write(address, self.fetched_data.source);
            }
            DestinationEnum::None => unimplemented!(),
        }
    }

    fn process_ldh(&mut self) {
        match self.fetched_data.destination {
            DestinationEnum::Register(register_type) => {
                if register_type != RegisterType::A {
                    unimplemented!();
                }
                self.set_register(register_type, self.fetched_data.source);
            }
            DestinationEnum::Address(address) => {
                if let ValueEnum::Data8(_) = self.fetched_data.source {
                    self.bus_write(address, self.fetched_data.source);
                } else {
                    unimplemented!();
                }
            }
            DestinationEnum::None => unimplemented!(),
        }
    }
    fn process_add(&mut self) {
        let z;
        let h;
        let c;
        match self.fetched_data.destination {
            DestinationEnum::Register(register_type) => match self.fetched_data.source {
                ValueEnum::None => unimplemented!(),
                ValueEnum::SignedData8(r8) => {
                    // ADD SP,r8
                    if register_type != RegisterType::SP {
                        unimplemented!();
                    }

                    z = Some(false);

                    h = Some(check_half_carry_relative(self.cpu_registers.sp, r8));
                    c = Some(check_carry_relative(self.cpu_registers.sp, r8));
                    self.cpu_registers.sp = add_relative(self.cpu_registers.sp, r8);

                    self.emu_cycles(1);
                }

                ValueEnum::Data8(d8) => {
                    if let ValueEnum::Data8(register_value) = self.get_register(register_type) {
                        c = match register_value.checked_add(d8) {
                            Some(_) => Some(false),
                            None => Some(true),
                        };
                        let sum = register_value + d8;
                        z = Some(sum == 0);
                        h = Some((register_value & 0x0F) + (d8 & 0x0F) > 0x0F);
                        self.set_register(register_type, ValueEnum::Data8(sum as u8));
                    } else {
                        unimplemented!();
                    }
                }

                ValueEnum::Data16(d16) => {
                    if let ValueEnum::Data16(register_value) = self.get_register(register_type) {
                        z = None;
                        h = Some((register_value & 0x0FFF) + (d16 & 0x0FFF) > 0x0FFF);
                        c = match register_value.checked_add(d16) {
                            Some(_) => Some(false),
                            None => Some(true),
                        };
                        self.set_register(register_type, ValueEnum::Data16(register_value + d16));

                        self.emu_cycles(1);
                    } else {
                        unimplemented!();
                    }
                }
            },
            DestinationEnum::Address(_) | DestinationEnum::None => unimplemented!(),
        }
        self.cpu_registers.set_flags(z, Some(false), h, c);
    }

    fn process_sbc(&mut self) {
        if let DestinationEnum::Register(register_type) = self.fetched_data.destination {
            if register_type != RegisterType::A {
                unimplemented!();
            }

            if let ValueEnum::Data8(value) = self.fetched_data.source {
                let carry = if self.cpu_registers.f.get_flag(Flags::C) {
                    1
                } else {
                    0
                };
                let result = self.cpu_registers.a - value - carry;

                let h = (self.cpu_registers.a & 0x0F) < (value & 0x0F) + carry;
                let c = self.cpu_registers.a.checked_sub(value + carry).is_none();

                self.cpu_registers
                    .set_flags(Some(result == 0), Some(true), Some(h), Some(c));
                self.cpu_registers.a = result;
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!()
        }
    }

    fn process_sub(&mut self) {
        if let ValueEnum::Data8(value) = self.fetched_data.source {
            let diff = self.cpu_registers.a - value;

            self.cpu_registers.set_flags(
                Some(diff == 0),
                Some(true),
                Some(self.cpu_registers.a & 0x0F < value & 0x0F),
                Some(self.cpu_registers.a < value),
            );

            self.cpu_registers.a = diff;
        } else {
            unimplemented!();
        }
    }

    fn goto_address(&mut self, address: u16, push_pc: bool) {
        if self.check_condition() {
            if push_pc {
                self.stack_push16(self.cpu_registers.pc)
            }
            self.cpu_registers.pc = address;
            self.emu_cycles(1);
        }
    }

    fn process_rst(&mut self) {
        if let Some(parameter) = self.current_instruction.parameter {
            self.goto_address(parameter as u16, false);
        } else {
            unimplemented!();
        }
    }

    fn process_jp(&mut self) {
        if let ValueEnum::Data16(address) = self.fetched_data.source {
            self.goto_address(address, false);
        } else {
            unimplemented!();
        }
    }

    fn process_jr(&mut self) {
        if let ValueEnum::SignedData8(relative) = self.fetched_data.source {
            let address = add_relative(self.cpu_registers.pc, relative);
            self.goto_address(address, false);
        } else {
            panic!("process_jr only accepts SignedData8");
        }
    }

    fn process_call(&mut self) {
        if let ValueEnum::Data16(address) = self.fetched_data.source {
            self.goto_address(address, true);
        } else {
            panic!("process_call only accepts Data16");
        }
    }

    fn process_ret(&mut self) {
        let should_ret;
        if self.current_instruction.condition == ConditionType::None {
            should_ret = true;
        } else {
            should_ret = self.check_condition();
            self.emu_cycles(1);
        }

        if should_ret {
            let address = self.stack_pop16();
            self.goto_address(address, false);
        }
    }

    fn process_reti(&mut self) {
        self.interrupt_master_enabled = true;
        self.process_ret();
    }

    fn process_push(&mut self) {
        if let ValueEnum::Data16(data) = self.fetched_data.source {
            self.stack_push16(data);
        } else {
            unimplemented!();
        }
        self.emu_cycles(1); // 16 bit register!
    }

    fn process_pop(&mut self) {
        if let DestinationEnum::Register(register) = self.fetched_data.destination {
            let value = ValueEnum::Data16(self.stack_pop16());
            self.set_register(register, value);
        } else {
            unimplemented!();
        }
    }

    fn process_inc(&mut self) {
        /* Should set Flags on 0x04 0x0C 0x14 0x1C 0x24 0x2C 0x34 0x3C
            00000100
            00001100
            00010100
            00011100
            00100100
            00101100
            00110100
            00111100
            opcode & 3F == opcode && opcode & 0x4 == 0x4
        */
        let new_value;
        match self.fetched_data.destination {
            DestinationEnum::Register(register) => {
                new_value = match self.cpu_registers.get_register(register) {
                    ValueEnum::SignedData8(_) | ValueEnum::None => unimplemented!(),
                    ValueEnum::Data8(value) => ValueEnum::Data8(value + 1),
                    ValueEnum::Data16(value) => {
                        self.emu_cycles(1);
                        ValueEnum::Data16(value + 1)
                    }
                };
                self.cpu_registers.set_register(register, new_value);
            }
            DestinationEnum::Address(address) => {
                new_value = ValueEnum::Data8(self.bus_read(address) + 1);
                self.bus_write(address, new_value);
            }
            DestinationEnum::None => unimplemented!(),
        }

        if self.current_opcode & 0x3F == self.current_opcode && self.current_opcode & 0x4 == 0x4 {
            let new_value_u8: u8 = new_value.try_into().unwrap();
            self.cpu_registers.set_flags(
                Some(new_value_u8 == 0),
                Some(false),
                Some(new_value_u8 & 0x0F == 0),
                None,
            )
        }
    }

    fn process_dec(&mut self) {
        /* Should set Flags on 0x05 0x0D 0x15 0x1D 0x25 0x2D 0x35 0x3D
            opcode & 0x3F && opcode & 6 == 4
        */
        let new_value;
        match self.fetched_data.destination {
            DestinationEnum::Register(register) => {
                new_value = match self.cpu_registers.get_register(register) {
                    ValueEnum::SignedData8(_) | ValueEnum::None => unimplemented!(),
                    ValueEnum::Data8(value) => ValueEnum::Data8(value - 1),
                    ValueEnum::Data16(value) => ValueEnum::Data16(value - 1),
                };
                self.cpu_registers.set_register(register, new_value);
            }
            DestinationEnum::Address(address) => {
                new_value = ValueEnum::Data8(self.bus_read(address) - 1);
                self.bus_write(address, new_value);
            }
            DestinationEnum::None => unimplemented!(),
        }

        if self.current_opcode & 0x3F == self.current_opcode && self.current_opcode & 0x6 == 0x4 {
            let new_value_u8: u8 = new_value.try_into().unwrap();
            self.cpu_registers.set_flags(
                Some(new_value_u8 == 0),
                Some(true),
                Some(new_value_u8 & 0x0F == 0x0F),
                None,
            )
        }
    }

    fn process_or(&mut self) {
        match self.fetched_data.source {
            ValueEnum::None | ValueEnum::SignedData8(_) | ValueEnum::Data16(_) => unimplemented!(),
            ValueEnum::Data8(value) => {
                self.cpu_registers.a |= value;
            }
        }

        self.cpu_registers.set_flags(
            Some(self.cpu_registers.a == 0),
            Some(false),
            Some(false),
            Some(false),
        )
    }

    fn process_xor(&mut self) {
        match self.fetched_data.source {
            ValueEnum::None | ValueEnum::SignedData8(_) | ValueEnum::Data16(_) => unimplemented!(),
            ValueEnum::Data8(value) => {
                self.cpu_registers.a ^= value;
            }
        }

        self.cpu_registers.set_flags(
            Some(self.cpu_registers.a == 0),
            Some(false),
            Some(false),
            Some(false),
        )
    }

    fn process_and(&mut self) {
        match self.fetched_data.source {
            ValueEnum::None | ValueEnum::SignedData8(_) | ValueEnum::Data16(_) => unimplemented!(),
            ValueEnum::Data8(value) => {
                self.cpu_registers.a &= value;
            }
        }

        self.cpu_registers.set_flags(
            Some(self.cpu_registers.a == 0),
            Some(false),
            Some(true),
            Some(false),
        )
    }

    fn process_cp(&mut self) {
        match self.fetched_data.source {
            ValueEnum::None | ValueEnum::SignedData8(_) | ValueEnum::Data16(_) => unimplemented!(),
            ValueEnum::Data8(value) => {
                let z = self.cpu_registers.a == value;
                let h = (self.cpu_registers.a & 0x0F) < (value & 0x0F);
                let c = self.cpu_registers.a < value;

                self.cpu_registers
                    .set_flags(Some(z), Some(true), Some(h), Some(c))
            }
        }
    }

    fn process_cb(&mut self) {
        if let ValueEnum::Data8(cb) = self.fetched_data.source {
            /*
            The lower 3 bits are the register index in RegistersLookup
            if high bits 00 The next 3 bits are the operations RLC, RRC, RL, RR, SLA, SRA, SWAP and SRL (if the )
            if high bits 01, the operations are BIT0, BIT1, BIT2, BIT3, BIT4, BIT5, BIT6 and BIT8
            if high bits 10, the operations are RES0, RES1, RES2, RES3, RES4, RES5, RES6 and RES8
            if high bits 11, the operations are SET0, SET1, SET2, SET3, SET4, SET5, SET6 and SET8
            */

            let register = REGISTERS_LOOKUP[cb as usize & 0x03];

            let bit_operation = (cb >> 6) & 0b11;

            let register_value = match self.get_register(register) {
                ValueEnum::None | ValueEnum::SignedData8(_) => unimplemented!(),
                ValueEnum::Data8(value) => value,
                ValueEnum::Data16(address) => {
                    // The only 16-bit register used here is HL, which contains the address of the value
                    self.emu_cycles(1); // 16-bit register
                    self.bus_read(address)
                }
            };

            let carry_flag: u8 = if self.cpu_registers.f.get_flag(Flags::C) {
                1
            } else {
                0
            };

            let bit_test_mask = 1 << ((cb >> 3) & 0b111);

            let mut result = None;

            let mut z_flag = None;
            let mut n_flag = None;
            let mut h_flag = None;
            let mut c_flag = None;

            self.emu_cycles(1); // decoding CB instruction requires another CPU cycle

            match bit_operation {
                // higher 2 bits
                0 => {
                    let carry;
                    // RLC, RRC, RL, RR, SLA, SRA, SWAP and SRL
                    let operation = (cb & 0x3F) >> 3;
                    let value = match operation {
                        0 => {
                            // RLC shift left and move upper most bit to right
                            // example 0b1010 -> 0b0101
                            let r = (register_value << 1) | (register_value >> 7);
                            carry = r & 0x1 == 0x1;
                            r
                        }
                        1 => {
                            // RRC
                            let r = (register_value >> 1) | (register_value << 7);
                            carry = (r & 0x80) == 0x80;
                            r
                        }
                        2 => {
                            // RL
                            carry = (register_value & 0x80) == 0x80;
                            (register_value << 1) | carry_flag
                        }
                        3 => {
                            // RR
                            carry = (register_value & 0x1) == 0x1;
                            (register_value >> 1) | (carry_flag << 7)
                        }
                        4 => {
                            // SLA
                            carry = (register_value & 0x80) == 0x80;
                            register_value << 1
                        }
                        5 => {
                            // SRA
                            carry = (register_value & 0x1) == 0x1;
                            ((register_value as i8) >> 1) as u8 // keep the sign bit
                        }
                        6 => {
                            // SWAP
                            carry = false;
                            (register_value >> 4) | (register_value << 4)
                        }
                        7 => {
                            // SRL
                            carry = (register_value & 0x1) == 0x1;
                            register_value >> 1
                        }
                        _ => unimplemented!(),
                    };
                    result = Some(value);

                    z_flag = Some(value == 0);
                    n_flag = Some(false);
                    h_flag = Some(false);
                    c_flag = Some(carry);
                }
                1 => {
                    // BIT
                    z_flag = Some((register_value & bit_test_mask) == bit_test_mask);
                    n_flag = Some(false);
                    h_flag = Some(true);
                }
                2 => {
                    //RST
                    result = Some(register_value & !bit_test_mask);
                }
                3 => {
                    //SET
                    result = Some(register_value | bit_test_mask);
                }
                _ => unimplemented!(),
            }

            match result {
                Some(value) => match register {
                    RegisterType::HL => {
                        if let ValueEnum::Data16(address) = self.get_register(register) {
                            self.bus_write(address, ValueEnum::Data8(value));
                        } else {
                            unimplemented!();
                        }
                    }
                    _ => {
                        self.set_register(register, ValueEnum::Data8(value));
                    }
                },
                None => {}
            }

            self.cpu_registers.set_flags(z_flag, n_flag, h_flag, c_flag);
        } else {
            unimplemented!();
        }
    }

    fn process_rra(&mut self) {
        let carry_flag: u8 = if self.cpu_registers.f.get_flag(Flags::C) {
            1
        } else {
            0
        };
        let result = (self.cpu_registers.a >> 1) | (carry_flag << 7);
        self.cpu_registers.set_flags(
            Some(false),
            Some(false),
            Some(false),
            Some(self.cpu_registers.a & 1 == 1),
        );
        self.cpu_registers.a = result;
    }

    fn process_cpl(&mut self) {
        self.cpu_registers.a = self.cpu_registers.a ^ 0xFF;
        self.cpu_registers
            .set_flags(None, Some(true), Some(true), None)
    }

    fn process_scf(&mut self) {
        self.cpu_registers
            .set_flags(None, Some(false), Some(false), Some(true));
    }

    fn process_ccf(&mut self) {
        self.cpu_registers.set_flags(
            None,
            Some(false),
            Some(false),
            Some(!self.cpu_registers.f.get_flag(Flags::C)),
        );
    }

    fn process_rlca(&mut self) {
        let carry = (self.cpu_registers.a >> 7) & 1;
        self.cpu_registers.a = (self.cpu_registers.a << 1) | carry;
        self.cpu_registers
            .set_flags(None, None, None, Some(carry == 1));
    }

    fn process_adc(&mut self) {
        let carry_flag: u8 = if self.cpu_registers.f.get_flag(Flags::C) {
            1
        } else {
            0
        };
        if let DestinationEnum::Register(register) = self.fetched_data.destination {
            if register != RegisterType::A {
                unimplemented!();
            }

            match self.fetched_data.source {
                ValueEnum::None | ValueEnum::Data16(_) | ValueEnum::SignedData8(_) => {
                    unimplemented!()
                }
                ValueEnum::Data8(value) => {
                    let result = self.cpu_registers.a + value + carry_flag;

                    let c = match self.cpu_registers.a.checked_add(value) {
                        Some(v) => match v.checked_add(carry_flag) {
                            Some(_) => false,
                            None => true,
                        },
                        None => true,
                    };
                    self.cpu_registers.set_flags(
                        Some(result == 0),
                        Some(false),
                        Some((self.cpu_registers.a & 0x0F) + (value & 0x0F) + carry_flag > 0x0F),
                        Some(c),
                    );
                    self.cpu_registers.a = result;
                }
            }
        } else {
            unimplemented!();
        }
    }

    fn process_daa(&mut self) {
        let half_carry_flag = self.cpu_registers.f.get_flag(Flags::H);
        let carry_flag = self.cpu_registers.f.get_flag(Flags::C);
        let n_flag = self.cpu_registers.f.get_flag(Flags::N);

        let mut result: u8 = 0;
        let mut new_carry: u8 = 0;

        if half_carry_flag || (!n_flag && (self.cpu_registers.a & 0x0F) > 9) {
            result = 6;
        }

        if carry_flag || (!n_flag && self.cpu_registers.a > 0x99) {
            result |= 0x60;
            new_carry = 1;
        }

        self.cpu_registers.a = if n_flag {
            self.cpu_registers.a - result
        } else {
            self.cpu_registers.a + result
        };

        self.cpu_registers.set_flags(
            Some(self.cpu_registers.a == 0),
            None,
            Some(false),
            Some(new_carry == 1),
        );
    }
}

const REGISTERS_LOOKUP: [RegisterType; 8] = [
    RegisterType::B,
    RegisterType::C,
    RegisterType::D,
    RegisterType::E,
    RegisterType::H,
    RegisterType::L,
    RegisterType::HL,
    RegisterType::A,
];

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
