use super::{
    execution_plan::{
        ArithmeticLogicUnitAction, CustomAction, ExecutionPlan, FetchAction, StoreAction,
    },
    instruction::ConditionType,
    Instruction, InstructionType, RegisterType,
};

impl<'i> From<u8> for Instruction<'i> {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self {
                description: "NOP",
                instruction_type: InstructionType::NOP,
                cpu_cycles: 4,
                ..Default::default()
            },
            0x01 => Self {
                description: "LD BC,d16",
                instruction_type: InstructionType::LD,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData16Bits,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::BC),
                ),
                ..Default::default()
            },
            0x02 => Self {
                description: "LD (BC),A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::BC),
                ),
                ..Default::default()
            },
            0x03 => Self {
                description: "INC BC",
                instruction_type: InstructionType::INC,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::BC),
                    ArithmeticLogicUnitAction::Inc16,
                    StoreAction::StoreRegister16Bits(RegisterType::BC),
                ),
                ..Default::default()
            },
            0x04 => Self {
                description: "INC B",
                cpu_cycles: 4,
                instruction_type: InstructionType::INC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::Inc,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },
            0x05 => Self {
                description: "DEC B",
                cpu_cycles: 4,
                instruction_type: InstructionType::DEC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::Dec,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },
            0x6 => Self {
                description: "LD B,d8",
                cpu_cycles: 8,
                instruction_type: InstructionType::LD,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },
            0x07 => unimplemented!("RLCA"),
            0x08 => Self {
                description: "LD (a16),SP",
                instruction_type: InstructionType::LD,
                cpu_cycles: 20,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::SP),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreAddress16Bits,
                ),
                ..Default::default()
            },
            0x09 => Self {
                description: "ADD HL,BC",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::BC),
                    ArithmeticLogicUnitAction::Add16(RegisterType::HL),
                    StoreAction::StoreRegister16Bits(RegisterType::HL),
                ),
                ..Default::default()
            },
            0x0A => Self {
                description: "LD A,(BC)",
                cpu_cycles: 8,
                instruction_type: InstructionType::LD,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::BC),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },
            0x0B => Self {
                description: "DEC BC",
                cpu_cycles: 8,
                instruction_type: InstructionType::DEC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::BC),
                    ArithmeticLogicUnitAction::Dec16,
                    StoreAction::StoreRegister16Bits(RegisterType::BC),
                ),
                ..Default::default()
            },
            0x0C => Self {
                description: "INC C",
                cpu_cycles: 4,
                instruction_type: InstructionType::INC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::Inc,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },
            0x0D => Self {
                description: "DEC C",
                cpu_cycles: 4,
                instruction_type: InstructionType::DEC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::Dec,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },
            0x0E => Self {
                description: "LD C,d8",
                cpu_cycles: 8,
                instruction_type: InstructionType::LD,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },
            0x0F => unimplemented!("RRCA"),

            0x10 => Self {
                description: "STOP 0",
                cpu_cycles: 4,
                instruction_type: InstructionType::STOP,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::STOP,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0x11 => Self {
                description: "LD DE,d16",
                cpu_cycles: 12,
                instruction_type: InstructionType::LD,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData16Bits,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::DE),
                ),
                ..Default::default()
            },

            0x12 => Self {
                description: "LD (DE),A",
                cpu_cycles: 8,
                instruction_type: InstructionType::LD,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::DE),
                ),
                ..Default::default()
            },
            0x13 => Self {
                description: "INC DE",
                cpu_cycles: 8,
                instruction_type: InstructionType::INC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::DE),
                    ArithmeticLogicUnitAction::Inc16,
                    StoreAction::StoreRegister16Bits(RegisterType::DE),
                ),
                ..Default::default()
            },

            0x14 => Self {
                description: "INC D",
                cpu_cycles: 4,
                instruction_type: InstructionType::INC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::Inc,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x15 => Self {
                description: "DEC D",
                cpu_cycles: 4,
                instruction_type: InstructionType::DEC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::Dec,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x16 => Self {
                description: "LD D,d8",
                cpu_cycles: 8,
                instruction_type: InstructionType::LD,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x17 => unimplemented!("RLA"),

            0x18 => Self {
                // This is equivalent to LD PC, PC + R8
                description: "JR r8",
                cpu_cycles: 12,
                instruction_type: InstructionType::JR,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16BitsWithOffset(RegisterType::PC),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },

            0x19 => Self {
                description: "ADD HL,DE",
                cpu_cycles: 8,
                instruction_type: InstructionType::ADD,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::DE),
                    ArithmeticLogicUnitAction::Add16(RegisterType::HL),
                    StoreAction::StoreRegister16Bits(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x1A => Self {
                description: "LD A,(DE)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::DE),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x1B => Self {
                description: "DEC DE",
                cpu_cycles: 8,
                instruction_type: InstructionType::DEC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::DE),
                    ArithmeticLogicUnitAction::Dec16,
                    StoreAction::StoreRegister16Bits(RegisterType::DE),
                ),
                ..Default::default()
            },

            0x1C => Self {
                description: "INC E",
                cpu_cycles: 4,
                instruction_type: InstructionType::INC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::Inc,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x1D => Self {
                description: "DEC E",
                cpu_cycles: 4,
                instruction_type: InstructionType::DEC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::Dec,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x1E => Self {
                description: "LD E,d8",
                cpu_cycles: 8,
                instruction_type: InstructionType::LD,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x1F => Self {
                description: "RRA",
                cpu_cycles: 4,
                instruction_type: InstructionType::RRA,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Rra,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },
            0x20 => Self {
                // JR NZ, R8 is equivalent to LD PC, PC + R8 if the zero flag is not set
                description: "JR NZ,r8",
                cpu_cycles: 12,
                cpu_cycles_condition_fails: 8,
                instruction_type: InstructionType::JR,
                condition: ConditionType::NZ,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16BitsWithOffset(RegisterType::PC),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0x21 => Self {
                description: "LD HL,d16",
                instruction_type: InstructionType::LD,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData16Bits,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::HL),
                ),
                ..Default::default()
            },
            0x22 => Self {
                description: "LD (HL+),A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirectAndIncrement(RegisterType::HL),
                ),
                ..Default::default()
            },
            0x23 => Self {
                description: "INC HL",
                cpu_cycles: 8,
                instruction_type: InstructionType::INC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::HL),
                    ArithmeticLogicUnitAction::Inc16,
                    StoreAction::StoreRegister16Bits(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x24 => Self {
                description: "INC H",
                instruction_type: InstructionType::INC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::Inc,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x25 => Self {
                description: "DEC H",
                instruction_type: InstructionType::DEC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::Dec,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x26 => Self {
                description: "LD H,d8",
                cpu_cycles: 8,
                instruction_type: InstructionType::LD,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x27 => Self {
                description: "DAA",
                instruction_type: InstructionType::DAA,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Daa,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },
            0x28 => Self {
                description: "JR Z,r8",
                instruction_type: InstructionType::JR,
                cpu_cycles: 12,
                cpu_cycles_condition_fails: 8,
                condition: ConditionType::Z,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16BitsWithOffset(RegisterType::PC),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0x29 => Self {
                description: "ADD HL,HL",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::HL),
                    ArithmeticLogicUnitAction::Add16(RegisterType::HL),
                    StoreAction::StoreRegister16Bits(RegisterType::HL),
                ),
                ..Default::default()
            },
            0x2A => Self {
                description: "LD A,(HL+)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirectAndIncrement(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x2B => Self {
                description: "DEC HL",
                instruction_type: InstructionType::DEC,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::HL),
                    ArithmeticLogicUnitAction::Dec16,
                    StoreAction::StoreRegister16Bits(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x2C => Self {
                description: "INC L",
                instruction_type: InstructionType::INC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::Inc,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x2D => Self {
                description: "DEC L",
                instruction_type: InstructionType::DEC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::Dec,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x2E => Self {
                description: "LD L,d8",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },
            0x2F => Self {
                description: "CPL",
                instruction_type: InstructionType::CPL,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Cpl,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },
            0x30 => Self {
                description: "JR NC,r8",
                instruction_type: InstructionType::JR,
                cpu_cycles: 12,
                cpu_cycles_condition_fails: 8,
                condition: ConditionType::NC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16BitsWithOffset(RegisterType::PC),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0x31 => Self {
                description: "LD SP,d16",
                instruction_type: InstructionType::LD,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData16Bits,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::SP),
                ),
                ..Default::default()
            },
            0x32 => Self {
                description: "LD (HL-),A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirectAndDecrement(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x33 => Self {
                description: "INC SP",
                instruction_type: InstructionType::INC,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::SP),
                    ArithmeticLogicUnitAction::Inc16,
                    StoreAction::StoreRegister16Bits(RegisterType::SP),
                ),
                ..Default::default()
            },

            0x34 => Self {
                description: "INC (HL)",
                instruction_type: InstructionType::INC,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::Inc,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },
            0x35 => Self {
                description: "DEC (HL)",
                instruction_type: InstructionType::DEC,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::Dec,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x36 => Self {
                description: "LD (HL),d8",
                instruction_type: InstructionType::LD,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x37 => Self {
                description: "SCF",
                instruction_type: InstructionType::SCF,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Scf,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0x38 => Self {
                description: "JR C,r8",
                instruction_type: InstructionType::JR,
                cpu_cycles: 12,
                cpu_cycles_condition_fails: 8,
                condition: ConditionType::C,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16BitsWithOffset(RegisterType::PC),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },

            0x39 => Self {
                description: "ADD HL,SP",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::SP),
                    ArithmeticLogicUnitAction::Add16(RegisterType::HL),
                    StoreAction::StoreRegister16Bits(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x3A => Self {
                description: "LD A,(HL-)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirectAndDecrement(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x3B => Self {
                description: "DEC SP",
                instruction_type: InstructionType::DEC,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::SP),
                    ArithmeticLogicUnitAction::Dec16,
                    StoreAction::StoreRegister16Bits(RegisterType::SP),
                ),
                ..Default::default()
            },

            0x3C => Self {
                description: "INC A",
                instruction_type: InstructionType::INC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Inc,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x3D => Self {
                description: "DEC A",
                instruction_type: InstructionType::DEC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Dec,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x3E => Self {
                description: "LD A,d8",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x3F => Self {
                description: "CCF",
                instruction_type: InstructionType::CCF,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Ccf,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0x40 => Self {
                description: "LD B,B",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },

            0x41 => Self {
                description: "LD B,C",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },

            0x42 => Self {
                description: "LD B,D",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },

            0x43 => Self {
                description: "LD B,E",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },

            0x44 => Self {
                description: "LD B,H",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },

            0x45 => Self {
                description: "LD B,L",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },

            0x46 => Self {
                description: "LD B,(HL)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },

            0x47 => Self {
                description: "LD B,A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::B),
                ),
                ..Default::default()
            },

            0x48 => Self {
                description: "LD C,B",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },

            0x49 => Self {
                description: "LD C,C",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },

            0x4A => Self {
                description: "LD C,D",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },

            0x4B => Self {
                description: "LD C,E",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },

            0x4C => Self {
                description: "LD C,H",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },

            0x4D => Self {
                description: "LD C,L",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },

            0x4E => Self {
                description: "LD C,(HL)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },

            0x4F => Self {
                description: "LD C,A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::C),
                ),
                ..Default::default()
            },

            0x50 => Self {
                description: "LD D,B",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x51 => Self {
                description: "LD D,C",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x52 => Self {
                description: "LD D,D",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x53 => Self {
                description: "LD D,E",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x54 => Self {
                description: "LD D,H",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x55 => Self {
                description: "LD D,L",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x56 => Self {
                description: "LD D,(HL)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x57 => Self {
                description: "LD D,A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::D),
                ),
                ..Default::default()
            },

            0x58 => Self {
                description: "LD E,B",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x59 => Self {
                description: "LD E,C",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x5A => Self {
                description: "LD E,D",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x5B => Self {
                description: "LD E,E",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x5C => Self {
                description: "LD E,H",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x5D => Self {
                description: "LD E,L",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x5E => Self {
                description: "LD E,(HL)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x5F => Self {
                description: "LD E,A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::E),
                ),
                ..Default::default()
            },

            0x60 => Self {
                description: "LD H,B",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x61 => Self {
                description: "LD H,C",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x62 => Self {
                description: "LD H,D",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x63 => Self {
                description: "LD H,E",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x64 => Self {
                description: "LD H,H",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x65 => Self {
                description: "LD H,L",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x66 => Self {
                description: "LD H,(HL)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x67 => Self {
                description: "LD H,A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::H),
                ),
                ..Default::default()
            },

            0x68 => Self {
                description: "LD L,B",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x69 => Self {
                description: "LD L,C",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x6A => Self {
                description: "LD L,D",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x6B => Self {
                description: "LD L,E",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x6C => Self {
                description: "LD L,H",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x6D => Self {
                description: "LD L,L",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x6E => Self {
                description: "LD L,(HL)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x6F => Self {
                description: "LD L,A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::L),
                ),
                ..Default::default()
            },

            0x70 => Self {
                description: "LD (HL),B",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x71 => Self {
                description: "LD (HL),C",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x72 => Self {
                description: "LD (HL),D",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x73 => Self {
                description: "LD (HL),E",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x74 => Self {
                description: "LD (HL),H",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x75 => Self {
                description: "LD (HL),L",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x76 => Self {
                description: "HALT",
                instruction_type: InstructionType::HALT,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::Halt,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0x77 => Self {
                description: "LD (HL),A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirect(RegisterType::HL),
                ),
                ..Default::default()
            },

            0x78 => Self {
                description: "LD A,B",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x79 => Self {
                description: "LD A,C",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x7A => Self {
                description: "LD A,D",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x7B => Self {
                description: "LD A,E",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x7C => Self {
                description: "LD A,H",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x7D => Self {
                description: "LD A,L",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x7E => Self {
                description: "LD A,(HL)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x7F => Self {
                description: "LD A,A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x80 => Self {
                description: "ADD A,B",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::Add(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x81 => Self {
                description: "ADD A,C",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::Add(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x82 => Self {
                description: "ADD A,D",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::Add(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x83 => Self {
                description: "ADD A,E",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::Add(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x84 => Self {
                description: "ADD A,H",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::Add(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x85 => Self {
                description: "ADD A,L",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::Add(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x86 => Self {
                description: "ADD A,(HL)",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::Add(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x87 => Self {
                description: "ADD A,A",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Add(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x88 => Self {
                description: "ADC A,B",
                instruction_type: InstructionType::ADC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::AddWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x89 => Self {
                description: "ADC A,C",
                instruction_type: InstructionType::ADC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::AddWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x8A => Self {
                description: "ADC A,D",
                instruction_type: InstructionType::ADC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::AddWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x8B => Self {
                description: "ADC A,E",
                instruction_type: InstructionType::ADC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::AddWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x8C => Self {
                description: "ADC A,H",
                instruction_type: InstructionType::ADC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::AddWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x8D => Self {
                description: "ADC A,L",
                instruction_type: InstructionType::ADC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::AddWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x8E => Self {
                description: "ADC A,(HL)",
                instruction_type: InstructionType::ADC,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::AddWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x8F => Self {
                description: "ADC A,A",
                instruction_type: InstructionType::ADC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::AddWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x90 => Self {
                description: "SUB B",
                instruction_type: InstructionType::SUB,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::Sub(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x91 => Self {
                description: "SUB C",
                instruction_type: InstructionType::SUB,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::Sub(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x92 => Self {
                description: "SUB D",
                instruction_type: InstructionType::SUB,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::Sub(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x93 => Self {
                description: "SUB E",
                instruction_type: InstructionType::SUB,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::Sub(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x94 => Self {
                description: "SUB H",
                instruction_type: InstructionType::SUB,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::Sub(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x95 => Self {
                description: "SUB L",
                instruction_type: InstructionType::SUB,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::Sub(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x96 => Self {
                description: "SUB (HL)",
                instruction_type: InstructionType::SUB,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::Sub(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x97 => Self {
                description: "SUB A",
                instruction_type: InstructionType::SUB,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Sub(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x98 => Self {
                description: "SBC A,B",
                instruction_type: InstructionType::SBC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::SubWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x99 => Self {
                description: "SBC A,C",
                instruction_type: InstructionType::SBC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::SubWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x9A => Self {
                description: "SBC A,D",
                instruction_type: InstructionType::SBC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::SubWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x9B => Self {
                description: "SBC A,E",
                instruction_type: InstructionType::SBC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::SubWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x9C => Self {
                description: "SBC A,H",
                instruction_type: InstructionType::SBC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::SubWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x9D => Self {
                description: "SBC A,L",
                instruction_type: InstructionType::SBC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::SubWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x9E => Self {
                description: "SBC A,(HL)",
                instruction_type: InstructionType::SBC,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::SubWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0x9F => Self {
                description: "SBC A,A",
                instruction_type: InstructionType::SBC,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::SubWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA0 => Self {
                description: "AND B",
                instruction_type: InstructionType::AND,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::And,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA1 => Self {
                description: "AND C",
                instruction_type: InstructionType::AND,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::And,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA2 => Self {
                description: "AND D",
                instruction_type: InstructionType::AND,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::And,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA3 => Self {
                description: "AND E",
                instruction_type: InstructionType::AND,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::And,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA4 => Self {
                description: "AND H",
                instruction_type: InstructionType::AND,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::And,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA5 => Self {
                description: "AND L",
                instruction_type: InstructionType::AND,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::And,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA6 => Self {
                description: "AND (HL)",
                instruction_type: InstructionType::AND,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::And,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA7 => Self {
                description: "AND A",
                instruction_type: InstructionType::AND,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::And,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA8 => Self {
                description: "XOR B",
                instruction_type: InstructionType::XOR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::Xor,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xA9 => Self {
                description: "XOR C",
                instruction_type: InstructionType::XOR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::Xor,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xAA => Self {
                description: "XOR D",
                instruction_type: InstructionType::XOR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::Xor,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xAB => Self {
                description: "XOR E",
                instruction_type: InstructionType::XOR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::Xor,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xAC => Self {
                description: "XOR H",
                instruction_type: InstructionType::XOR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::Xor,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xAD => Self {
                description: "XOR L",
                instruction_type: InstructionType::XOR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::Xor,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xAE => Self {
                description: "XOR (HL)",
                instruction_type: InstructionType::XOR,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::Xor,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xAF => Self {
                description: "XOR A",
                instruction_type: InstructionType::XOR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Xor,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xB0 => Self {
                description: "OR B",
                instruction_type: InstructionType::OR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::Or,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xB1 => Self {
                description: "OR C",
                instruction_type: InstructionType::OR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::Or,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xB2 => Self {
                description: "OR D",
                instruction_type: InstructionType::OR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::Or,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xB3 => Self {
                description: "OR E",
                instruction_type: InstructionType::OR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::Or,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xB4 => Self {
                description: "OR H",
                instruction_type: InstructionType::OR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::Or,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xB5 => Self {
                description: "OR L",
                instruction_type: InstructionType::OR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::Or,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xB6 => Self {
                description: "OR (HL)",
                instruction_type: InstructionType::OR,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::Or,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xB7 => Self {
                description: "OR A",
                instruction_type: InstructionType::OR,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Or,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xB8 => Self {
                description: "CP B",
                instruction_type: InstructionType::CP,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::B),
                    ArithmeticLogicUnitAction::Cp,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xB9 => Self {
                description: "CP C",
                instruction_type: InstructionType::CP,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::C),
                    ArithmeticLogicUnitAction::Cp,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xBA => Self {
                description: "CP D",
                instruction_type: InstructionType::CP,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::D),
                    ArithmeticLogicUnitAction::Cp,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xBB => Self {
                description: "CP E",
                instruction_type: InstructionType::CP,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::E),
                    ArithmeticLogicUnitAction::Cp,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xBC => Self {
                description: "CP H",
                instruction_type: InstructionType::CP,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::H),
                    ArithmeticLogicUnitAction::Cp,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xBD => Self {
                description: "CP L",
                instruction_type: InstructionType::CP,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::L),
                    ArithmeticLogicUnitAction::Cp,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xBE => Self {
                description: "CP (HL)",
                instruction_type: InstructionType::CP,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirect(RegisterType::HL),
                    ArithmeticLogicUnitAction::Cp,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xBF => Self {
                description: "CP A",
                instruction_type: InstructionType::CP,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::Cp,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xC0 => Self {
                description: "RET NZ",
                instruction_type: InstructionType::RET,
                cpu_cycles: 20,
                cpu_cycles_condition_fails: 8,
                condition: ConditionType::NZ,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::PopPC,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xC1 => Self {
                description: "POP BC",
                instruction_type: InstructionType::POP,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchStack,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::BC),
                ),
                ..Default::default()
            },
            0xC2 => Self {
                description: "JP NZ,a16",
                instruction_type: InstructionType::JP,
                cpu_cycles: 16,
                cpu_cycles_condition_fails: 12,
                condition: ConditionType::NZ,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData16Bits,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xC3 => Self {
                // JP a16 is equivalent to LD PC, d16
                // The next two bytes after the instruction bytecode 0xC3 constitute the jump address
                description: "JP a16",
                instruction_type: InstructionType::JP,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData16Bits, // no FetchAddress becaue we are not fetching the
                    // value stored in that address
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xC4 => Self {
                description: "CALL NZ,a16",
                instruction_type: InstructionType::CALL,
                cpu_cycles: 24,
                cpu_cycles_condition_fails: 12,
                condition: ConditionType::NZ,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::FetchData16Bits,
                    CustomAction::PushPC,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xC5 => Self {
                description: "PUSH BC",
                instruction_type: InstructionType::PUSH,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::BC),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreStack,
                ),
                ..Default::default()
            },

            0xC6 => Self {
                description: "ADD A,d8",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::Add(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xC7 => Self {
                description: "RST 00H",
                instruction_type: InstructionType::RST,
                cpu_cycles: 16,
                parameter: Some(0x00),
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::RST,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xC8 => Self {
                description: "RET Z",
                instruction_type: InstructionType::RET,
                cpu_cycles: 20,
                cpu_cycles_condition_fails: 8,
                condition: ConditionType::Z,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::PopPC,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xC9 => Self {
                description: "RET",
                instruction_type: InstructionType::RET,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::PopPC,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xCA => Self {
                description: "JP Z,a16",
                instruction_type: InstructionType::JP,
                cpu_cycles: 16,
                cpu_cycles_condition_fails: 12,
                condition: ConditionType::Z,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData16Bits,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xCB => Self {
                description: "PREFIX CB",
                instruction_type: InstructionType::CB,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::FetchData,
                    CustomAction::PrefixCB,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xCC => Self {
                description: "CALL Z,a16",
                instruction_type: InstructionType::CALL,
                cpu_cycles: 24,
                cpu_cycles_condition_fails: 12,
                condition: ConditionType::Z,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::FetchData16Bits,
                    CustomAction::PushPC,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xCD => Self {
                description: "CALL a16",
                instruction_type: InstructionType::CALL,
                cpu_cycles: 24,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::FetchData16Bits,
                    CustomAction::PushPC,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },

            0xCE => Self {
                description: "ADC A,d8",
                instruction_type: InstructionType::ADC,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::AddWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xCF => Self {
                description: "RST 08H",
                instruction_type: InstructionType::RST,
                cpu_cycles: 16,
                parameter: Some(0x08),
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::RST,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xD0 => Self {
                description: "RET NC",
                instruction_type: InstructionType::RET,
                cpu_cycles: 20,
                cpu_cycles_condition_fails: 8,
                condition: ConditionType::NC,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::PopPC,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xD1 => Self {
                description: "POP DE",
                instruction_type: InstructionType::POP,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchStack,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::DE),
                ),
                ..Default::default()
            },
            0xD2 => Self {
                description: "JP NC,a16",
                instruction_type: InstructionType::JP,
                cpu_cycles: 16,
                cpu_cycles_condition_fails: 12,
                condition: ConditionType::NC,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData16Bits,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xD3 => unimplemented!("INVALID"),
            0xD4 => Self {
                description: "CALL NC,a16",
                instruction_type: InstructionType::CALL,
                cpu_cycles: 24,
                cpu_cycles_condition_fails: 12,
                condition: ConditionType::NC,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::FetchData16Bits,
                    CustomAction::PushPC,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xD5 => Self {
                description: "PUSH DE",
                instruction_type: InstructionType::PUSH,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::DE),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreStack,
                ),
                ..Default::default()
            },

            0xD6 => Self {
                description: "SUB d8",
                instruction_type: InstructionType::SUB,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::Sub(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xD7 => Self {
                description: "RST 10H",
                instruction_type: InstructionType::RST,
                cpu_cycles: 16,
                parameter: Some(0x10),
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::RST,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xD8 => Self {
                description: "RET C",
                instruction_type: InstructionType::RET,
                cpu_cycles: 20,
                cpu_cycles_condition_fails: 8,
                condition: ConditionType::C,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::PopPC,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xD9 => Self {
                description: "RETI",
                instruction_type: InstructionType::RETI,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::RETI,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xDA => Self {
                description: "JP C,a16",
                instruction_type: InstructionType::JP,
                cpu_cycles: 16,
                cpu_cycles_condition_fails: 12,
                condition: ConditionType::C,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData16Bits,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xDB => unimplemented!("INVALID"),
            0xDC => Self {
                description: "CALL C,a16",
                instruction_type: InstructionType::CALL,
                cpu_cycles: 24,
                cpu_cycles_condition_fails: 12,
                condition: ConditionType::C,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::FetchData16Bits,
                    CustomAction::PushPC,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xDD => unimplemented!("INVALID"),

            0xDE => Self {
                description: "SBC A,d8",
                instruction_type: InstructionType::SBC,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::SubWithCarry(RegisterType::A),
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xDF => Self {
                description: "RST 18H",
                instruction_type: InstructionType::RST,
                cpu_cycles: 16,
                parameter: Some(0x18),
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::RST,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xE0 => Self {
                description: "LDH (a8),A",
                instruction_type: InstructionType::LDH,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreAddressZeroPage,
                ),
                ..Default::default()
            },

            0xE1 => Self {
                description: "POP HL",
                instruction_type: InstructionType::POP,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchStack,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::HL),
                ),
                ..Default::default()
            },
            0xE2 => Self {
                description: "LD (C),A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreIndirectZeroPage(RegisterType::C),
                ),
                ..Default::default()
            },

            0xE3 => unimplemented!("INVALID"),
            0xE4 => unimplemented!("INVALID"),

            0xE5 => Self {
                description: "PUSH HL",
                instruction_type: InstructionType::PUSH,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreStack,
                ),
                ..Default::default()
            },

            0xE6 => Self {
                description: "AND d8",
                instruction_type: InstructionType::AND,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::And,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xE7 => Self {
                description: "RST 20H",
                instruction_type: InstructionType::RST,
                cpu_cycles: 16,
                parameter: Some(0x20),
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::RST,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xE8 => Self {
                description: "ADD SP,r8",
                instruction_type: InstructionType::ADD,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::AddRelative(RegisterType::SP),
                    StoreAction::StoreRegister16Bits(RegisterType::SP),
                ),
                ..Default::default()
            },

            0xE9 => Self {
                description: "JP (HL)",
                instruction_type: InstructionType::JP,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::PC),
                ),
                ..Default::default()
            },
            0xEA => Self {
                description: "LD (a16),A",
                instruction_type: InstructionType::LD,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister(RegisterType::A),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreAddress,
                ),
                ..Default::default()
            },

            0xEB => unimplemented!("INVALID"),
            0xEC => unimplemented!("INVALID"),
            0xED => unimplemented!("INVALID"),
            0xEE => Self {
                description: "XOR d8",
                instruction_type: InstructionType::XOR,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::Xor,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xEF => Self {
                description: "RST 28H",
                instruction_type: InstructionType::RST,
                cpu_cycles: 16,
                parameter: Some(0x28),
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::RST,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xF0 => Self {
                description: "LDH A,(a8)",
                instruction_type: InstructionType::LDH,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchAddressZeroPage,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xF1 => Self {
                description: "POP AF",
                instruction_type: InstructionType::POP,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchStack,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::AF),
                ),
                ..Default::default()
            },
            0xF2 => Self {
                description: "LD A,(C)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchIndirectZeroPage(RegisterType::C),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xF3 => Self {
                description: "DI",
                instruction_type: InstructionType::DI,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::DI,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xF4 => unimplemented!("INVALID"),
            0xF5 => Self {
                description: "PUSH AF",
                instruction_type: InstructionType::PUSH,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::AF),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreStack,
                ),
                ..Default::default()
            },

            0xF6 => Self {
                description: "OR d8",
                instruction_type: InstructionType::OR,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::Or,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xF7 => Self {
                description: "RST 30H",
                instruction_type: InstructionType::RST,
                cpu_cycles: 16,
                parameter: Some(0x30),
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::RST,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xF8 => Self {
                description: "LD HL,SP+r8",
                instruction_type: InstructionType::LD,
                cpu_cycles: 12,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16BitsWithOffset(RegisterType::SP),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::HL),
                ),
                ..Default::default()
            },

            0xF9 => Self {
                description: "LD SP,HL",
                instruction_type: InstructionType::LD,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchRegister16Bits(RegisterType::HL),
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister16Bits(RegisterType::SP),
                ),
                ..Default::default()
            },

            0xFA => Self {
                description: "LD A,(a16)",
                instruction_type: InstructionType::LD,
                cpu_cycles: 16,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchAddress,
                    ArithmeticLogicUnitAction::None,
                    StoreAction::StoreRegister(RegisterType::A),
                ),
                ..Default::default()
            },

            0xFB => Self {
                description: "EI",
                instruction_type: InstructionType::EI,
                cpu_cycles: 4,
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::EI,
                    StoreAction::None,
                ),
                ..Default::default()
            },
            0xFC => unimplemented!("INVALID"),
            0xFD => unimplemented!("INVALID"),

            0xFE => Self {
                description: "CP d8",
                instruction_type: InstructionType::CP,
                cpu_cycles: 8,
                execution_plan: ExecutionPlan::new(
                    FetchAction::FetchData,
                    ArithmeticLogicUnitAction::Cp,
                    StoreAction::None,
                ),
                ..Default::default()
            },

            0xFF => Self {
                description: "RST 38H",
                instruction_type: InstructionType::RST,
                cpu_cycles: 16,
                parameter: Some(0x38),
                execution_plan: ExecutionPlan::with_custom_action(
                    FetchAction::None,
                    CustomAction::RST,
                    StoreAction::None,
                ),
                ..Default::default()
            },
        }
    }
}
