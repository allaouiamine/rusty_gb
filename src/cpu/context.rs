use std::sync::Arc;
use std::sync::Mutex;

use crate::bus::Bus;
use crate::cpu::custom_operations::CpuExtension;
use crate::cpu::execution_plan::ArithmeticLogicUnitAction;

use super::execution_plan::FetchAction;
use super::execution_plan::StoreAction;
use super::instruction::ConditionType;
use super::instruction::InstructionType;
use super::registers::CpuRegisters;
use super::registers::Flags;
use super::types::InterruptType;
use super::RegisterType;

use super::instruction::Instruction;
use super::util::add_relative;
use super::util::check_carry_relative;
use super::util::check_half_carry_relative;
use super::ValueEnum;

pub struct CpuContext<'a> {
    pub bus: Arc<Mutex<dyn Bus>>,
    pub cpu_registers: CpuRegisters,
    pub current_instruction: Instruction<'a>,
    pub old_pc: u16,
    pub current_opcode: u8,
    halted: bool,
    pub ticks: usize,
    interrupt_master_enabled_next_instruction: bool,
    interrupt_master_enabled: bool,

    pub last_written_address: Option<u16>,
    pub dma_done: bool,
}

impl<'a> CpuContext<'a> {
    pub fn new(bus: Arc<Mutex<dyn Bus>>) -> Self {
        Self {
            bus,
            cpu_registers: CpuRegisters::new(),
            current_instruction: Instruction::default(),
            old_pc: 0x100,
            current_opcode: 0,
            halted: false,
            ticks: 0,
            interrupt_master_enabled_next_instruction: false,
            interrupt_master_enabled: false,
            last_written_address: None,

            dma_done: false,
        }
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn enable_master_interrupt_next_instruction(&mut self) {
        self.interrupt_master_enabled_next_instruction = true;
    }

    pub fn enable_master_interrupt(&mut self) {
        self.interrupt_master_enabled = true;
    }

    pub fn disable_master_interrupt(&mut self) {
        self.interrupt_master_enabled = false;
    }

    fn get_interrupt_flags_register(&self) -> u8 {
        self.bus.lock().unwrap().bus_read(0xFF0F)
    }

    fn set_interrupt_flags_register(&mut self, value: u8) {
        self.bus.lock().unwrap().bus_write(0xFF0F, value);
    }

    fn get_interrupt_enable_register(&self) -> u8 {
        self.bus.lock().unwrap().bus_read(0xFFFF)
    }

    pub fn bus_read(&mut self, address: u16) -> u8 {
        self.bus.lock().unwrap().bus_read(address)
    }

    pub fn bus_read16(&mut self, address: u16) -> u16 {
        let lo = self.bus_read(address) as u16;
        let hi = self.bus_read(address + 1) as u16;
        lo | (hi << 8)
    }

    pub fn bus_write(&mut self, address: u16, value: u8) {
        self.bus.lock().unwrap().bus_write(address, value);
        self.last_written_address = Some(address);
    }

    pub fn bus_write_16(&mut self, address: u16, value: u16) {
        self.bus_write(address, value as u8);
        self.bus_write(address + 1, (value >> 8) as u8);
    }

    pub fn stack_push(&mut self, data: u8) {
        self.cpu_registers.sp -= 1;
        self.bus_write(self.cpu_registers.sp, data);
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
        self.current_opcode = self.bus_read(self.cpu_registers.pc);
        self.current_instruction = Instruction::from(self.current_opcode);
        self.cpu_registers.pc += 1;
    }

    pub fn push_pc(&mut self) {
        self.stack_push16(self.cpu_registers.pc);
    }

    fn interrupt_handle(&mut self, address: u16) {
        println!("Running interrupt handler for address: {:04X}", address);
        self.push_pc();
        self.cpu_registers.pc = address;
    }

    fn cpu_handle_interrupts(&mut self) {
        // The register 0xFF0F shows which interrupts are requested
        let interrupt_flags = self.get_interrupt_flags_register();

        // The register 0xFFFF shows which interrupts are enabled
        let interrupt_enable = self.get_interrupt_enable_register();

        // The AND operation will show which interrupts are requested and enabled
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

    pub fn cpu_step(&mut self) -> anyhow::Result<usize> {
        let current_ticks = self.ticks;
        self.dma_done = false;
        self.last_written_address = None;

        self.old_pc = self.cpu_registers.pc;
        if !self.halted {
            self.fetch_instruction();
            self.execute_current_instruction()?;
        } else {
            self.emu_cycles(4);
            if self.get_interrupt_flags_register() != 0 {
                self.halted = false;
            }
        }

        if self.interrupt_master_enabled {
            self.cpu_handle_interrupts();
            self.interrupt_master_enabled_next_instruction = false;
        }

        if self.interrupt_master_enabled_next_instruction {
            // This is used to enable the interrupts after the next instruction
            self.interrupt_master_enabled = true;
        }

        Ok(current_ticks - self.ticks)
    }

    fn fetch_data(&mut self) -> anyhow::Result<ValueEnum> {
        Ok(
            match self.current_instruction.execution_plan.get_fetch_action() {
                FetchAction::None => ValueEnum::None,
                FetchAction::FetchData => ValueEnum::Data8(self.get_next_pc_value()),
                FetchAction::FetchSignedData => {
                    ValueEnum::SignedData8(self.get_next_pc_value() as i8)
                }
                FetchAction::FetchData16Bits => ValueEnum::Data16(self.get_next_pc_value16()),
                FetchAction::FetchAddress => {
                    let address = self.get_next_pc_value16();
                    ValueEnum::Data8(self.bus_read(address))
                }
                FetchAction::FetchAddressZeroPage => {
                    let mut address = self.get_next_pc_value() as u16;
                    address |= 0xFF00;
                    ValueEnum::Data8(self.bus_read(address))
                }
                FetchAction::FetchRegister(register_type) => {
                    ValueEnum::Data8(self.cpu_registers.get_register(register_type))
                }
                FetchAction::FetchRegister16Bits(register_type_16) => {
                    ValueEnum::Data16(self.cpu_registers.get_register_16(register_type_16))
                }
                FetchAction::FetchRegister16BitsWithOffset(register_type_16) => {
                    let register_value = if register_type_16 == &RegisterType::PC {
                        self.cpu_registers.pc + 1
                    } else {
                        self.cpu_registers.get_register_16(register_type_16)
                    };
                    let offset_unsigned = self.get_next_pc_value();
                    let offset = offset_unsigned as i8;

                    let h = Some(check_half_carry_relative(register_value, offset));
                    let c = Some(check_carry_relative(register_value, offset_unsigned));
                    let sum = add_relative(register_value, offset);

                    if self.current_instruction.instruction_type != InstructionType::JR {
                        self.cpu_registers.set_flags(Some(false), Some(false), h, c);
                    }
                    ValueEnum::Data16(sum)
                }
                FetchAction::FetchIndirect(register_type_16) => {
                    let address = self.cpu_registers.get_register_16(register_type_16);
                    ValueEnum::Data8(self.bus_read(address))
                }
                FetchAction::FetchIndirectZeroPage(register_type) => {
                    // This is GameBoy specific implementation of the zero page
                    // which is technically the last 256 bytes of the memory, not the first 256
                    // the 8 bit address is stored in the C register and then we add 0xFF00 to it
                    match register_type {
                        RegisterType::C => {
                            let address = self.cpu_registers.get_register(register_type) as u16;
                            ValueEnum::Data8(self.bus_read(address | 0xFF00))
                        }
                        _ => anyhow::bail!("Only C register is allowed for zero page addressing"),
                    }
                }
                FetchAction::FetchIndirectAndIncrement(register_type_16) => {
                    match register_type_16 {
                        RegisterType::HL => {
                            let address = self.cpu_registers.get_register_16(register_type_16);
                            self.cpu_registers
                                .set_register_16(register_type_16, address + 1);
                            let value = self.bus_read(address);
                            ValueEnum::Data8(value)
                        }
                        _ => anyhow::bail!("Only HL register is allowed for Indirect increment"),
                    }
                }
                FetchAction::FetchIndirectAndDecrement(register_type_16) => {
                    match register_type_16 {
                        RegisterType::HL => {
                            let address = self.cpu_registers.get_register_16(register_type_16);
                            self.cpu_registers
                                .set_register_16(register_type_16, address - 1);
                            let value = self.bus_read(address);
                            ValueEnum::Data8(value)
                        }
                        _ => anyhow::bail!("Only HL register is allowed for Indirect decrement"),
                    }
                }
                FetchAction::FetchStack => {
                    let value = self.stack_pop16();
                    ValueEnum::Data16(value)
                }
            },
        )
    }

    fn store_data(&mut self, value: ValueEnum) -> anyhow::Result<()> {
        match self.current_instruction.execution_plan.get_store_action() {
            StoreAction::None => {}
            StoreAction::StoreRegister(register_type) => self
                .cpu_registers
                .set_register(register_type, value.try_into()?),
            StoreAction::StoreRegister16Bits(register_type_16) => self
                .cpu_registers
                .set_register_16(register_type_16, value.try_into()?),
            StoreAction::StoreStack => {
                self.stack_push16(value.try_into()?);
            }
            StoreAction::StoreIndirect(register_16) => {
                let address = self.cpu_registers.get_register_16(register_16);
                self.bus_write(address, value.try_into()?);
            }
            StoreAction::StoreIndirectZeroPage(register_type) => match register_type {
                RegisterType::C => {
                    let mut address = self.cpu_registers.get_register(register_type) as u16;
                    address |= 0xFF00;
                    self.bus_write(address, value.try_into()?);
                }
                _ => anyhow::bail!("Only C register is allowed for zero page addressing"),
            },
            StoreAction::StoreIndirectAndIncrement(register_type_16) => match register_type_16 {
                RegisterType::HL => {
                    let address = self.cpu_registers.get_register_16(register_type_16);
                    self.cpu_registers
                        .set_register_16(register_type_16, address + 1);
                    self.bus_write(address, value.try_into()?);
                }
                _ => anyhow::bail!("Only HL register is allowed for Indirect increment"),
            },
            StoreAction::StoreIndirectAndDecrement(register_type_16) => match register_type_16 {
                RegisterType::HL => {
                    let address = self.cpu_registers.get_register_16(register_type_16);
                    self.cpu_registers
                        .set_register_16(register_type_16, address - 1);
                    self.bus_write(address, value.try_into()?);
                }
                _ => anyhow::bail!("Only HL register is allowed for Indirect decrement"),
            },
            StoreAction::StoreAddress => {
                let address = self.get_next_pc_value16();
                self.bus_write(address, value.try_into()?);
            }
            StoreAction::StoreAddress16Bits => {
                let address = self.get_next_pc_value16();
                self.bus_write_16(address, value.try_into()?); // only used n 0x80 :LD (a16),SP
            }
            StoreAction::StoreAddressZeroPage => {
                let mut address = self.get_next_pc_value() as u16;
                address |= 0xFF00;
                self.bus_write(address, value.try_into()?);
            }
        }
        Ok(())
    }

    fn dbg_test_rom(&mut self) {
        //println!("{}", self);
        let mut bus = self.bus.lock().unwrap();
        bus.dbg_update();
        //bus.dbg_print();
    }

    fn execute_alu(&mut self, fetched_data: ValueEnum) -> anyhow::Result<ValueEnum> {
        let execution_plan = self.current_instruction.execution_plan;
        let action = execution_plan.get_arithmetic_logic_unit_action();
        if action == &ArithmeticLogicUnitAction::None {
            Ok(fetched_data)
        } else {
            let output_data = action
                .get_operation()
                .execute(fetched_data, &self.cpu_registers)?;
            self.cpu_registers.set_flags(
                output_data.z,
                output_data.n,
                output_data.h,
                output_data.c,
            );
            Ok(output_data.value)
        }
    }

    pub fn execute_current_instruction(&mut self) -> anyhow::Result<()> {
        let fetched_data = self.fetch_data()?;

        self.dbg_test_rom();

        if self.check_condition() {
            self.execute_custom(fetched_data)?;
            let data_to_store = self.execute_alu(fetched_data)?;
            self.store_data(data_to_store)?;
            self.emu_cycles(self.current_instruction.cpu_cycles);
            assert_ne!(self.current_instruction.cpu_cycles, 0);
        } else {
            self.emu_cycles(self.current_instruction.cpu_cycles_condition_fails);
            assert_ne!(self.current_instruction.cpu_cycles_condition_fails, 0);
        }
        Ok(())
    }

    pub fn get_next_pc_value(&mut self) -> u8 {
        let value = self.bus_read(self.cpu_registers.pc);
        self.cpu_registers.pc += 1;
        value
    }

    pub fn get_next_pc_value16(&mut self) -> u16 {
        let lo = self.get_next_pc_value() as u16;
        let hi = self.get_next_pc_value() as u16;
        (hi << 8) | lo
    }

    fn request_interrupt(&mut self, interrupt_type: InterruptType) {
        let interrupt_flags = self.get_interrupt_flags_register();
        self.set_interrupt_flags_register(interrupt_flags | (interrupt_type as u8));
    }

    pub fn emu_cycles(&mut self, ticks: usize) {
        let machine_ticks = ticks / 4;
        for _ in 0..machine_ticks {
            for _ in 0..4 {
                self.ticks += 1;

                let interrupt_option = self.bus.lock().unwrap().timer_tick();

                if let Some(interrupt) = interrupt_option {
                    println!("Interrupt requested: {:?}", interrupt);
                    self.request_interrupt(interrupt);
                }

                let interrupts = self.bus.lock().unwrap().ppu_tick();
                for interrupt in interrupts {
                    println!("Interrupt requested: {:?}", interrupt);
                    self.request_interrupt(interrupt);
                }
            }
            self.bus.lock().unwrap().dma_tick();
        }
    }

    fn check_condition(&mut self) -> bool {
        if self.current_instruction.condition == ConditionType::None {
            return true;
        }

        let z = self.cpu_registers.f.get_flag(Flags::Z);
        let c = self.cpu_registers.f.get_flag(Flags::C);

        let condition = match self.current_instruction.condition {
            ConditionType::C => c,
            ConditionType::Z => z,
            ConditionType::NC => !c,
            ConditionType::NZ => !z,
            ConditionType::None => true,
        };

        condition
    }
}
