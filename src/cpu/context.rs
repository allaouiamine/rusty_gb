use crate::bus::Bus;
use crate::cpu::execution_plan::ArithmeticLogicUnitAction;

use super::execution_plan::FetchAction;
use super::execution_plan::StoreAction;
use super::RegisterType;
use super::registers::CpuRegisters;

use super::instruction::Instruction;
use super::util::add_relative;
use super::util::check_carry_relative;
use super::util::check_half_carry_relative;
use super::ValueEnum;

pub struct CpuContext<'a> {
    pub bus: Bus<'a>,
    pub cpu_registers: CpuRegisters,
    pub current_instruction: Instruction<'a>,
    pub old_pc: u16,
    pub current_opcode: u8,
    halted: bool,
    pub ticks: usize,
    interrupt_master_enabled: bool,
    enabling_ime: bool,

    pub last_written_address: Option<u16>,
    pub dma_done: bool,

    ly: u8,
}

impl<'a> CpuContext<'a> {
    pub fn new(rom_file: &'a str) -> Self {
        let bus = Bus::new(rom_file);
        Self {
            bus,
            cpu_registers: CpuRegisters::new(),
            current_instruction: Instruction::default(),
            old_pc: 0x100,
            current_opcode: 0,
            halted: false,
            // stepping: false,
            ticks: 0,
            interrupt_master_enabled: false,
            enabling_ime: false,
            last_written_address: None,

            dma_done: false,
            ly: 0,
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
        println!("Running interrupt handler for address: {:04X}", address);
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

    fn return_and_inc_ly(&mut self) -> u8 {
        let ly = self.ly;
        self.ly += 1;
        ly
    }

    pub fn bus_read(&mut self, address: u16) -> u8 {
        self.emu_cycles(1);
        if address == 0xFF44 {
            self.return_and_inc_ly()
        } else {
            self.bus.bus_read(address)
        }
    }

    pub fn bus_read16(&mut self, address: u16) -> u16 {
        self.emu_cycles(2);
        self.bus.bus_read16(address)
    }

    pub fn bus_write(&mut self, address: u16, value: u8) {
        self.bus.bus_write8(address, value);
        self.emu_cycles(1);
        self.last_written_address = Some(address);
    }

    pub fn bus_write_16(&mut self, address: u16, value: u16) {
        self.bus.bus_write16(address, value);
        self.emu_cycles(2);
        self.last_written_address = Some(address);
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
        let pc: u16 = self.cpu_registers.pc;
        self.current_opcode = self.bus_read(pc);

        self.current_instruction = Instruction::from(self.current_opcode);

        self.cpu_registers.pc += 1;
    }

    pub fn cpu_step(&mut self) -> bool {
        self.dma_done = false;
        self.last_written_address = None;

        self.old_pc = self.cpu_registers.pc;
        if !self.halted {
            self.fetch_instruction();
            self.execute_current_instruction().unwrap();
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

    fn fetch_data(&mut self) -> anyhow::Result<ValueEnum> {
        Ok(
            match self.current_instruction.execution_plan.get_fetch_action() {
                FetchAction::None => ValueEnum::None,
                FetchAction::FetchData => ValueEnum::Data8(self.get_next_pc_value()),
                FetchAction::FetchSignedData => {
                    ValueEnum::SignedData8(self.get_next_pc_value() as i8)
                }
                FetchAction::FetchData16Bits => ValueEnum::Data16(self.get_next_pc_value16()),
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
                    if self.current_opcode != 0xF8 {
                        anyhow::bail!("Only used in 0xF8 - 'LD HL,SP+r8'");
                    }
                    match register_type_16 {
                        RegisterType::SP => {
                            let offset = self.get_next_pc_value() as i8;
                            let sp_value = self.cpu_registers.sp;

                            let h = Some(check_half_carry_relative(sp_value, offset));
                            let c = Some(check_carry_relative(sp_value, offset));
                            let sum = add_relative(sp_value, offset);

                            self.cpu_registers.set_flags(Some(false), Some(false), h, c);
                            ValueEnum::Data16(sum)
                        }
                        _ => anyhow::bail!("Only SP register is allowed to fetch with an offset"),
                    }
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
                FetchAction::FetchAddress => todo!("FetchAction::FetchAddress"),
            },
        )
    }

    fn store_data(&mut self, value: ValueEnum) -> anyhow::Result<()> {
        match self.current_instruction.execution_plan.get_store_actions() {
            StoreAction::None => todo!("StoreAction::None"),
            StoreAction::StoreRegister(register_type) => self
                .cpu_registers
                .set_register(register_type, value.try_into()?),
            StoreAction::StoreRegister16Bits(register_type_16) => self
                .cpu_registers
                .set_register_16(register_type_16, value.try_into()?),
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

    pub fn execute_current_instruction(&mut self) -> anyhow::Result<()> {
        let execution_plan = self.current_instruction.execution_plan;
        let fetched_data = self.fetch_data()?;

        // println!("{}", self);
        // self.bus.dbg_update();
        // self.bus.dbg_print();
        let output_value = match execution_plan.get_arithmetic_logic_unit_actions() {
            ArithmeticLogicUnitAction::None => fetched_data, // LD instruction for example
            other => {
                unimplemented!("Unimplemented action: {:?}", other);
            }
        };
        self.store_data(output_value)
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

    fn emu_cycles(&mut self, ticks: usize) {
        for _ in 0..ticks {
            for _ in 0..4 {
                self.ticks += 1;

                if let Some(interrupt) = self.bus.io.timer.timer_tick() {
                    self.request_interrupt(interrupt);
                }
            }
            self.dma_done = self.bus.dma_tick();
        }
    }

    pub fn execute(&mut self) {}
}

/*
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
*/
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
