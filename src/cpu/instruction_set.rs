use super::{
    instruction::{ConditionType, Operand},
    Instruction, InstructionType, RegisterType,
};

pub struct InstructionSet {
    instructions: Vec<Instruction<'static>>,
}

impl InstructionSet {
    pub fn new() -> Self {
        // completed all LD

        let mut instructions = Vec::with_capacity(0x100);

        for _ in 0..0x100 {
            instructions.push(Instruction::instruction_no_operands(
                "NONE",
                InstructionType::NONE,
            ));
        }

        //TODO! initialize the instruction vector!
        instructions[0x00] = Instruction::instruction_no_operands("NOP", InstructionType::NOP);

        instructions[0x01] = Instruction::instruction_2_operands(
            "LD BC,d16",
            InstructionType::LD,
            Operand::Register(RegisterType::BC),
            Operand::D16,
        );
        instructions[0x02] = Instruction::instruction_2_operands(
            "LD (BC),A",
            InstructionType::LD,
            Operand::Indirect(RegisterType::BC),
            Operand::Register(RegisterType::A),
        );

        instructions[0x03] = Instruction::instruction_1_operand(
            "INC BC",
            InstructionType::INC,
            Operand::Register(RegisterType::BC),
        );

        instructions[0x04] = Instruction::instruction_1_operand(
            "INC B",
            InstructionType::INC,
            Operand::Register(RegisterType::B),
        );

        instructions[0x05] = Instruction::instruction_1_operand(
            "DEC B",
            InstructionType::DEC,
            Operand::Register(RegisterType::B),
        );

        instructions[0x06] = Instruction::instruction_2_operands(
            "LD B,d8",
            InstructionType::LD,
            Operand::Register(RegisterType::B),
            Operand::D8,
        );

        instructions[0x07] = Instruction::instruction_no_operands("RLCA", InstructionType::RLCA);

        instructions[0x08] = Instruction::instruction_2_operands(
            "LD (a16),SP",
            InstructionType::LD,
            Operand::A16Indirect,
            Operand::Register(RegisterType::SP),
        );

        instructions[0x09] = Instruction::instruction_2_operands(
            "ADD HL,BC",
            InstructionType::ADD,
            Operand::Register(RegisterType::HL),
            Operand::Register(RegisterType::BC),
        );

        instructions[0x0A] = Instruction::instruction_2_operands(
            "LD A,(BC)",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Indirect(RegisterType::BC),
        );

        instructions[0x0B] = Instruction::instruction_1_operand(
            "DEC BC",
            InstructionType::DEC,
            Operand::Register(RegisterType::BC),
        );

        instructions[0x0C] = Instruction::instruction_1_operand(
            "INC C",
            InstructionType::INC,
            Operand::Register(RegisterType::C),
        );

        instructions[0x0D] = Instruction::instruction_1_operand(
            "DEC C",
            InstructionType::DEC,
            Operand::Register(RegisterType::C),
        );

        instructions[0x0E] = Instruction::instruction_2_operands(
            "LD C,d8",
            InstructionType::LD,
            Operand::Register(RegisterType::C),
            Operand::D8,
        );

        instructions[0x0F] = Instruction::instruction_no_operands("RRCA", InstructionType::RRCA);

        instructions[0x10] = Instruction::instruction_no_operands("STOP", InstructionType::STOP);
        instructions[0x11] = Instruction::instruction_2_operands(
            "LD DE,d16",
            InstructionType::LD,
            Operand::Register(RegisterType::DE),
            Operand::D16,
        );
        instructions[0x12] = Instruction::instruction_2_operands(
            "LD (DE),A",
            InstructionType::LD,
            Operand::Indirect(RegisterType::DE),
            Operand::Register(RegisterType::A),
        );

        instructions[0x13] = Instruction::instruction_1_operand(
            "INC DE",
            InstructionType::INC,
            Operand::Register(RegisterType::DE),
        );

        instructions[0x14] = Instruction::instruction_1_operand(
            "INC D",
            InstructionType::INC,
            Operand::Register(RegisterType::D),
        );

        instructions[0x15] = Instruction::instruction_1_operand(
            "DEC D",
            InstructionType::DEC,
            Operand::Register(RegisterType::D),
        );

        instructions[0x16] = Instruction::instruction_2_operands(
            "LD D,d8",
            InstructionType::LD,
            Operand::Register(RegisterType::D),
            Operand::D8,
        );

        instructions[0x17] = Instruction::instruction_no_operands("RLA", InstructionType::RLA);

        instructions[0x18] =
            Instruction::instruction_1_operand("JR r8", InstructionType::JR, Operand::R8);

        instructions[0x19] = Instruction::instruction_2_operands(
            "ADD HL,DE",
            InstructionType::ADD,
            Operand::Register(RegisterType::HL),
            Operand::Register(RegisterType::DE),
        );

        instructions[0x1A] = Instruction::instruction_2_operands(
            "LD A,(DE)",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Indirect(RegisterType::DE),
        );

        instructions[0x1B] = Instruction::instruction_1_operand(
            "DEC DE",
            InstructionType::DEC,
            Operand::Register(RegisterType::DE),
        );

        instructions[0x1C] = Instruction::instruction_1_operand(
            "INC E",
            InstructionType::INC,
            Operand::Register(RegisterType::E),
        );
        instructions[0x1D] = Instruction::instruction_1_operand(
            "DEC E",
            InstructionType::DEC,
            Operand::Register(RegisterType::E),
        );
        instructions[0x1E] = Instruction::instruction_2_operands(
            "LD E,d8",
            InstructionType::LD,
            Operand::Register(RegisterType::E),
            Operand::D8,
        );

        instructions[0x1F] = Instruction::instruction_no_operands("RRA", InstructionType::RRA);
        instructions[0x20] = Instruction::instruction_1_operand_with_condition(
            "JR NZ,r8",
            InstructionType::JR,
            Operand::R8,
            ConditionType::NZ,
        );
        instructions[0x21] = Instruction::instruction_2_operands(
            "LD HL,d16",
            InstructionType::LD,
            Operand::Register(RegisterType::HL),
            Operand::D16,
        );
        instructions[0x22] = Instruction::instruction_2_operands(
            "LD (HL+),A",
            InstructionType::LD,
            Operand::IndirectIncrementHL,
            Operand::Register(RegisterType::A),
        );
        instructions[0x23] = Instruction::instruction_1_operand(
            "INC HL",
            InstructionType::INC,
            Operand::Register(RegisterType::HL),
        );

        instructions[0x24] = Instruction::instruction_1_operand(
            "INC H",
            InstructionType::INC,
            Operand::Register(RegisterType::H),
        );
        instructions[0x25] = Instruction::instruction_1_operand(
            "DEC H",
            InstructionType::DEC,
            Operand::Register(RegisterType::H),
        );
        instructions[0x26] = Instruction::instruction_2_operands(
            "LD H,d8",
            InstructionType::LD,
            Operand::Register(RegisterType::H),
            Operand::D8,
        );

        instructions[0x27] = Instruction::instruction_no_operands("DAA", InstructionType::DAA);

        instructions[0x28] = Instruction::instruction_1_operand_with_condition(
            "JR Z,r8",
            InstructionType::JR,
            Operand::R8,
            ConditionType::Z,
        );

        instructions[0x29] = Instruction::instruction_2_operands(
            "ADD HL,HL",
            InstructionType::ADD,
            Operand::Register(RegisterType::HL),
            Operand::Register(RegisterType::HL),
        );

        instructions[0x2A] = Instruction::instruction_2_operands(
            "LD A,(HL+)",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::IndirectIncrementHL,
        );
        instructions[0x2B] = Instruction::instruction_1_operand(
            "DEC HL",
            InstructionType::DEC,
            Operand::Register(RegisterType::HL),
        );
        instructions[0x2C] = Instruction::instruction_1_operand(
            "INC L",
            InstructionType::INC,
            Operand::Register(RegisterType::L),
        );
        instructions[0x2D] = Instruction::instruction_1_operand(
            "DEC L",
            InstructionType::DEC,
            Operand::Register(RegisterType::L),
        );
        instructions[0x2E] = Instruction::instruction_2_operands(
            "LD L,d8",
            InstructionType::LD,
            Operand::Register(RegisterType::L),
            Operand::D8,
        );

        instructions[0x2F] = Instruction::instruction_no_operands("CPL", InstructionType::CPL);
        instructions[0x30] = Instruction::instruction_1_operand_with_condition(
            "JR NC,r8",
            InstructionType::JR,
            Operand::R8,
            ConditionType::NC,
        );
        instructions[0x31] = Instruction::instruction_2_operands(
            "LD SP,d16",
            InstructionType::LD,
            Operand::Register(RegisterType::SP),
            Operand::D16,
        );
        instructions[0x32] = Instruction::instruction_2_operands(
            "LD (HL-),A",
            InstructionType::LD,
            Operand::IndirectDecrementHL,
            Operand::Register(RegisterType::A),
        );
        instructions[0x33] = Instruction::instruction_1_operand(
            "INC SP",
            InstructionType::INC,
            Operand::Register(RegisterType::SP),
        );

        instructions[0x34] = Instruction::instruction_1_operand(
            "INC (HL)",
            InstructionType::INC,
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x35] = Instruction::instruction_1_operand(
            "DEC (HL)",
            InstructionType::DEC,
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x36] = Instruction::instruction_2_operands(
            "LD (HL),d8",
            InstructionType::LD,
            Operand::Indirect(RegisterType::HL),
            Operand::D8,
        );
        instructions[0x37] = Instruction::instruction_no_operands("SCF", InstructionType::SCF);
        instructions[0x38] = Instruction::instruction_1_operand_with_condition(
            "JR C,r8",
            InstructionType::JR,
            Operand::R8,
            ConditionType::C,
        );
        instructions[0x39] = Instruction::instruction_2_operands(
            "ADD HL,SP",
            InstructionType::ADD,
            Operand::Register(RegisterType::HL),
            Operand::Register(RegisterType::SP),
        );

        instructions[0x3A] = Instruction::instruction_2_operands(
            "LD A,(HL-)",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::IndirectDecrementHL,
        );
        instructions[0x3B] = Instruction::instruction_1_operand(
            "DEC SP",
            InstructionType::DEC,
            Operand::Register(RegisterType::SP),
        );
        instructions[0x3C] = Instruction::instruction_1_operand(
            "INC A",
            InstructionType::INC,
            Operand::Register(RegisterType::A),
        );
        instructions[0x3D] = Instruction::instruction_1_operand(
            "DEC A",
            InstructionType::DEC,
            Operand::Register(RegisterType::A),
        );
        instructions[0x3E] = Instruction::instruction_2_operands(
            "LD A,d8",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::D8,
        );

        instructions[0x3F] = Instruction::instruction_no_operands("CCF", InstructionType::CCF);
        instructions[0x40] = Instruction::instruction_2_operands(
            "LD B,B",
            InstructionType::LD,
            Operand::Register(RegisterType::B),
            Operand::Register(RegisterType::B),
        );
        instructions[0x41] = Instruction::instruction_2_operands(
            "LD B,C",
            InstructionType::LD,
            Operand::Register(RegisterType::B),
            Operand::Register(RegisterType::C),
        );
        instructions[0x42] = Instruction::instruction_2_operands(
            "LD B,D",
            InstructionType::LD,
            Operand::Register(RegisterType::B),
            Operand::Register(RegisterType::D),
        );
        instructions[0x43] = Instruction::instruction_2_operands(
            "LD B,E",
            InstructionType::LD,
            Operand::Register(RegisterType::B),
            Operand::Register(RegisterType::E),
        );
        instructions[0x44] = Instruction::instruction_2_operands(
            "LD B,H",
            InstructionType::LD,
            Operand::Register(RegisterType::B),
            Operand::Register(RegisterType::H),
        );
        instructions[0x45] = Instruction::instruction_2_operands(
            "LD B,L",
            InstructionType::LD,
            Operand::Register(RegisterType::B),
            Operand::Register(RegisterType::L),
        );
        instructions[0x46] = Instruction::instruction_2_operands(
            "LD B,(HL)",
            InstructionType::LD,
            Operand::Register(RegisterType::B),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x47] = Instruction::instruction_2_operands(
            "LD B,A",
            InstructionType::LD,
            Operand::Register(RegisterType::B),
            Operand::Register(RegisterType::A),
        );
        instructions[0x48] = Instruction::instruction_2_operands(
            "LD C,B",
            InstructionType::LD,
            Operand::Register(RegisterType::C),
            Operand::Register(RegisterType::B),
        );
        instructions[0x49] = Instruction::instruction_2_operands(
            "LD C,C",
            InstructionType::LD,
            Operand::Register(RegisterType::C),
            Operand::Register(RegisterType::C),
        );
        instructions[0x4A] = Instruction::instruction_2_operands(
            "LD C,D",
            InstructionType::LD,
            Operand::Register(RegisterType::C),
            Operand::Register(RegisterType::D),
        );
        instructions[0x4B] = Instruction::instruction_2_operands(
            "LD C,E",
            InstructionType::LD,
            Operand::Register(RegisterType::C),
            Operand::Register(RegisterType::E),
        );
        instructions[0x4C] = Instruction::instruction_2_operands(
            "LD C,H",
            InstructionType::LD,
            Operand::Register(RegisterType::C),
            Operand::Register(RegisterType::H),
        );
        instructions[0x4D] = Instruction::instruction_2_operands(
            "LD C,L",
            InstructionType::LD,
            Operand::Register(RegisterType::C),
            Operand::Register(RegisterType::L),
        );
        instructions[0x4E] = Instruction::instruction_2_operands(
            "LD C,(HL)",
            InstructionType::LD,
            Operand::Register(RegisterType::C),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x4F] = Instruction::instruction_2_operands(
            "LD C,A",
            InstructionType::LD,
            Operand::Register(RegisterType::C),
            Operand::Register(RegisterType::A),
        );

        instructions[0x50] = Instruction::instruction_2_operands(
            "LD D,B",
            InstructionType::LD,
            Operand::Register(RegisterType::D),
            Operand::Register(RegisterType::B),
        );
        instructions[0x51] = Instruction::instruction_2_operands(
            "LD D,C",
            InstructionType::LD,
            Operand::Register(RegisterType::D),
            Operand::Register(RegisterType::C),
        );
        instructions[0x52] = Instruction::instruction_2_operands(
            "LD D,D",
            InstructionType::LD,
            Operand::Register(RegisterType::D),
            Operand::Register(RegisterType::D),
        );
        instructions[0x53] = Instruction::instruction_2_operands(
            "LD D,E",
            InstructionType::LD,
            Operand::Register(RegisterType::D),
            Operand::Register(RegisterType::E),
        );
        instructions[0x54] = Instruction::instruction_2_operands(
            "LD D,H",
            InstructionType::LD,
            Operand::Register(RegisterType::D),
            Operand::Register(RegisterType::H),
        );
        instructions[0x55] = Instruction::instruction_2_operands(
            "LD D,L",
            InstructionType::LD,
            Operand::Register(RegisterType::D),
            Operand::Register(RegisterType::L),
        );
        instructions[0x56] = Instruction::instruction_2_operands(
            "LD D,(HL)",
            InstructionType::LD,
            Operand::Register(RegisterType::D),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x57] = Instruction::instruction_2_operands(
            "LD D,A",
            InstructionType::LD,
            Operand::Register(RegisterType::D),
            Operand::Register(RegisterType::A),
        );
        instructions[0x58] = Instruction::instruction_2_operands(
            "LD E,B",
            InstructionType::LD,
            Operand::Register(RegisterType::E),
            Operand::Register(RegisterType::B),
        );
        instructions[0x59] = Instruction::instruction_2_operands(
            "LD E,C",
            InstructionType::LD,
            Operand::Register(RegisterType::E),
            Operand::Register(RegisterType::C),
        );
        instructions[0x5A] = Instruction::instruction_2_operands(
            "LD E,D",
            InstructionType::LD,
            Operand::Register(RegisterType::E),
            Operand::Register(RegisterType::D),
        );
        instructions[0x5B] = Instruction::instruction_2_operands(
            "LD E,E",
            InstructionType::LD,
            Operand::Register(RegisterType::E),
            Operand::Register(RegisterType::E),
        );
        instructions[0x5C] = Instruction::instruction_2_operands(
            "LD E,H",
            InstructionType::LD,
            Operand::Register(RegisterType::E),
            Operand::Register(RegisterType::H),
        );
        instructions[0x5D] = Instruction::instruction_2_operands(
            "LD E,L",
            InstructionType::LD,
            Operand::Register(RegisterType::E),
            Operand::Register(RegisterType::L),
        );
        instructions[0x5E] = Instruction::instruction_2_operands(
            "LD E,(HL)",
            InstructionType::LD,
            Operand::Register(RegisterType::E),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x5F] = Instruction::instruction_2_operands(
            "LD E,A",
            InstructionType::LD,
            Operand::Register(RegisterType::E),
            Operand::Register(RegisterType::A),
        );

        instructions[0x60] = Instruction::instruction_2_operands(
            "LD H,B",
            InstructionType::LD,
            Operand::Register(RegisterType::H),
            Operand::Register(RegisterType::B),
        );
        instructions[0x61] = Instruction::instruction_2_operands(
            "LD H,C",
            InstructionType::LD,
            Operand::Register(RegisterType::H),
            Operand::Register(RegisterType::C),
        );
        instructions[0x62] = Instruction::instruction_2_operands(
            "LD H,D",
            InstructionType::LD,
            Operand::Register(RegisterType::H),
            Operand::Register(RegisterType::D),
        );
        instructions[0x63] = Instruction::instruction_2_operands(
            "LD H,E",
            InstructionType::LD,
            Operand::Register(RegisterType::H),
            Operand::Register(RegisterType::E),
        );
        instructions[0x64] = Instruction::instruction_2_operands(
            "LD H,H",
            InstructionType::LD,
            Operand::Register(RegisterType::H),
            Operand::Register(RegisterType::H),
        );
        instructions[0x65] = Instruction::instruction_2_operands(
            "LD H,L",
            InstructionType::LD,
            Operand::Register(RegisterType::H),
            Operand::Register(RegisterType::L),
        );
        instructions[0x66] = Instruction::instruction_2_operands(
            "LD H,(HL)",
            InstructionType::LD,
            Operand::Register(RegisterType::H),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x67] = Instruction::instruction_2_operands(
            "LD H,A",
            InstructionType::LD,
            Operand::Register(RegisterType::H),
            Operand::Register(RegisterType::A),
        );
        instructions[0x68] = Instruction::instruction_2_operands(
            "LD L,B",
            InstructionType::LD,
            Operand::Register(RegisterType::L),
            Operand::Register(RegisterType::B),
        );
        instructions[0x69] = Instruction::instruction_2_operands(
            "LD L,C",
            InstructionType::LD,
            Operand::Register(RegisterType::L),
            Operand::Register(RegisterType::C),
        );
        instructions[0x6A] = Instruction::instruction_2_operands(
            "LD L,D",
            InstructionType::LD,
            Operand::Register(RegisterType::L),
            Operand::Register(RegisterType::D),
        );
        instructions[0x6B] = Instruction::instruction_2_operands(
            "LD L,E",
            InstructionType::LD,
            Operand::Register(RegisterType::L),
            Operand::Register(RegisterType::E),
        );
        instructions[0x6C] = Instruction::instruction_2_operands(
            "LD L,H",
            InstructionType::LD,
            Operand::Register(RegisterType::L),
            Operand::Register(RegisterType::H),
        );
        instructions[0x6D] = Instruction::instruction_2_operands(
            "LD L,L",
            InstructionType::LD,
            Operand::Register(RegisterType::L),
            Operand::Register(RegisterType::L),
        );
        instructions[0x6E] = Instruction::instruction_2_operands(
            "LD L,(HL)",
            InstructionType::LD,
            Operand::Register(RegisterType::L),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x6F] = Instruction::instruction_2_operands(
            "LD L,A",
            InstructionType::LD,
            Operand::Register(RegisterType::L),
            Operand::Register(RegisterType::A),
        );

        instructions[0x70] = Instruction::instruction_2_operands(
            "LD (HL),B",
            InstructionType::LD,
            Operand::Indirect(RegisterType::HL),
            Operand::Register(RegisterType::B),
        );
        instructions[0x71] = Instruction::instruction_2_operands(
            "LD (HL),C",
            InstructionType::LD,
            Operand::Indirect(RegisterType::HL),
            Operand::Register(RegisterType::C),
        );
        instructions[0x72] = Instruction::instruction_2_operands(
            "LD (HL),D",
            InstructionType::LD,
            Operand::Indirect(RegisterType::HL),
            Operand::Register(RegisterType::D),
        );
        instructions[0x73] = Instruction::instruction_2_operands(
            "LD (HL),E",
            InstructionType::LD,
            Operand::Indirect(RegisterType::HL),
            Operand::Register(RegisterType::E),
        );
        instructions[0x74] = Instruction::instruction_2_operands(
            "LD (HL),H",
            InstructionType::LD,
            Operand::Indirect(RegisterType::HL),
            Operand::Register(RegisterType::H),
        );
        instructions[0x75] = Instruction::instruction_2_operands(
            "LD (HL),L",
            InstructionType::LD,
            Operand::Indirect(RegisterType::HL),
            Operand::Register(RegisterType::L),
        );

        instructions[0x76] = Instruction::instruction_no_operands("HALT", InstructionType::HALT);

        instructions[0x77] = Instruction::instruction_2_operands(
            "LD (HL),A",
            InstructionType::LD,
            Operand::Indirect(RegisterType::HL),
            Operand::Register(RegisterType::A),
        );
        instructions[0x78] = Instruction::instruction_2_operands(
            "LD A,B",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::B),
        );
        instructions[0x79] = Instruction::instruction_2_operands(
            "LD A,C",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::C),
        );
        instructions[0x7A] = Instruction::instruction_2_operands(
            "LD A,D",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::D),
        );
        instructions[0x7B] = Instruction::instruction_2_operands(
            "LD A,E",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::E),
        );
        instructions[0x7C] = Instruction::instruction_2_operands(
            "LD A,H",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::H),
        );
        instructions[0x7D] = Instruction::instruction_2_operands(
            "LD A,L",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::L),
        );
        instructions[0x7E] = Instruction::instruction_2_operands(
            "LD A,(HL)",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x7F] = Instruction::instruction_2_operands(
            "LD A,A",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::A),
        );

        instructions[0x80] = Instruction::instruction_2_operands(
            "ADD A,B",
            InstructionType::ADD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::B),
        );
        instructions[0x81] = Instruction::instruction_2_operands(
            "ADD A,C",
            InstructionType::ADD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::C),
        );
        instructions[0x82] = Instruction::instruction_2_operands(
            "ADD A,D",
            InstructionType::ADD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::D),
        );
        instructions[0x83] = Instruction::instruction_2_operands(
            "ADD A,E",
            InstructionType::ADD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::E),
        );
        instructions[0x84] = Instruction::instruction_2_operands(
            "ADD A,H",
            InstructionType::ADD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::H),
        );
        instructions[0x85] = Instruction::instruction_2_operands(
            "ADD A,L",
            InstructionType::ADD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::L),
        );
        instructions[0x86] = Instruction::instruction_2_operands(
            "ADD A,(HL)",
            InstructionType::ADD,
            Operand::Register(RegisterType::A),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x87] = Instruction::instruction_2_operands(
            "ADD A,A",
            InstructionType::ADD,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::A),
        );

        instructions[0x88] = Instruction::instruction_2_operands(
            "ADC A,B",
            InstructionType::ADC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::B),
        );
        instructions[0x89] = Instruction::instruction_2_operands(
            "ADC A,C",
            InstructionType::ADC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::C),
        );
        instructions[0x8A] = Instruction::instruction_2_operands(
            "ADC A,D",
            InstructionType::ADC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::D),
        );
        instructions[0x8B] = Instruction::instruction_2_operands(
            "ADC A,E",
            InstructionType::ADC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::E),
        );
        instructions[0x8C] = Instruction::instruction_2_operands(
            "ADC A,H",
            InstructionType::ADC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::H),
        );
        instructions[0x8D] = Instruction::instruction_2_operands(
            "ADC A,L",
            InstructionType::ADC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::L),
        );
        instructions[0x8E] = Instruction::instruction_2_operands(
            "ADC A,(HL)",
            InstructionType::ADC,
            Operand::Register(RegisterType::A),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x8F] = Instruction::instruction_2_operands(
            "ADC A,A",
            InstructionType::ADC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::A),
        );

        instructions[0x90] = Instruction::instruction_1_operand(
            "SUB B",
            InstructionType::SUB,
            Operand::Register(RegisterType::B),
        );
        instructions[0x91] = Instruction::instruction_1_operand(
            "SUB C",
            InstructionType::SUB,
            Operand::Register(RegisterType::C),
        );
        instructions[0x92] = Instruction::instruction_1_operand(
            "SUB D",
            InstructionType::SUB,
            Operand::Register(RegisterType::D),
        );
        instructions[0x93] = Instruction::instruction_1_operand(
            "SUB E",
            InstructionType::SUB,
            Operand::Register(RegisterType::E),
        );
        instructions[0x94] = Instruction::instruction_1_operand(
            "SUB H",
            InstructionType::SUB,
            Operand::Register(RegisterType::H),
        );

        instructions[0x95] = Instruction::instruction_1_operand(
            "SUB L",
            InstructionType::SUB,
            Operand::Register(RegisterType::L),
        );

        instructions[0x96] = Instruction::instruction_1_operand(
            "SUB (HL)",
            InstructionType::SUB,
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x97] = Instruction::instruction_1_operand(
            "SUB A",
            InstructionType::SUB,
            Operand::Register(RegisterType::A),
        );

        instructions[0x98] = Instruction::instruction_2_operands(
            "SBC A B",
            InstructionType::SBC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::B),
        );
        instructions[0x99] = Instruction::instruction_2_operands(
            "SBC A C",
            InstructionType::SBC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::C),
        );
        instructions[0x9A] = Instruction::instruction_2_operands(
            "SBC A D",
            InstructionType::SBC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::D),
        );
        instructions[0x9B] = Instruction::instruction_2_operands(
            "SBC A E",
            InstructionType::SBC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::E),
        );
        instructions[0x9C] = Instruction::instruction_2_operands(
            "SBC A H",
            InstructionType::SBC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::H),
        );
        instructions[0x9D] = Instruction::instruction_2_operands(
            "SBC A L",
            InstructionType::SBC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::L),
        );
        instructions[0x9E] = Instruction::instruction_2_operands(
            "SBC A (HL)",
            InstructionType::SBC,
            Operand::Register(RegisterType::A),
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0x9F] = Instruction::instruction_2_operands(
            "SBC A A",
            InstructionType::SBC,
            Operand::Register(RegisterType::A),
            Operand::Register(RegisterType::A),
        );

        instructions[0xA0] = Instruction::instruction_1_operand(
            "AND B",
            InstructionType::AND,
            Operand::Register(RegisterType::B),
        );

        instructions[0xA1] = Instruction::instruction_1_operand(
            "AND C",
            InstructionType::AND,
            Operand::Register(RegisterType::C),
        );
        instructions[0xA2] = Instruction::instruction_1_operand(
            "AND D",
            InstructionType::AND,
            Operand::Register(RegisterType::D),
        );
        instructions[0xA3] = Instruction::instruction_1_operand(
            "AND E",
            InstructionType::AND,
            Operand::Register(RegisterType::E),
        );
        instructions[0xA4] = Instruction::instruction_1_operand(
            "AND H",
            InstructionType::AND,
            Operand::Register(RegisterType::H),
        );
        instructions[0xA5] = Instruction::instruction_1_operand(
            "AND L",
            InstructionType::AND,
            Operand::Register(RegisterType::L),
        );
        instructions[0xA6] = Instruction::instruction_1_operand(
            "AND (HL)",
            InstructionType::AND,
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0xA7] = Instruction::instruction_1_operand(
            "AND A",
            InstructionType::AND,
            Operand::Register(RegisterType::A),
        );

        instructions[0xA8] = Instruction::instruction_1_operand(
            "XOR B",
            InstructionType::XOR,
            Operand::Register(RegisterType::B),
        );

        instructions[0xA9] = Instruction::instruction_1_operand(
            "XOR C",
            InstructionType::XOR,
            Operand::Register(RegisterType::C),
        );
        instructions[0xAA] = Instruction::instruction_1_operand(
            "XOR D",
            InstructionType::XOR,
            Operand::Register(RegisterType::D),
        );
        instructions[0xAB] = Instruction::instruction_1_operand(
            "XOR E",
            InstructionType::XOR,
            Operand::Register(RegisterType::E),
        );
        instructions[0xAC] = Instruction::instruction_1_operand(
            "XOR H",
            InstructionType::XOR,
            Operand::Register(RegisterType::H),
        );
        instructions[0xAD] = Instruction::instruction_1_operand(
            "XOR L",
            InstructionType::XOR,
            Operand::Register(RegisterType::L),
        );
        instructions[0xAE] = Instruction::instruction_1_operand(
            "XOR (HL)",
            InstructionType::XOR,
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0xAF] = Instruction::instruction_1_operand(
            "XOR A",
            InstructionType::XOR,
            Operand::Register(RegisterType::A),
        );

        instructions[0xB0] = Instruction::instruction_1_operand(
            "OR B",
            InstructionType::OR,
            Operand::Register(RegisterType::B),
        );

        instructions[0xB1] = Instruction::instruction_1_operand(
            "OR C",
            InstructionType::OR,
            Operand::Register(RegisterType::C),
        );
        instructions[0xB2] = Instruction::instruction_1_operand(
            "OR D",
            InstructionType::OR,
            Operand::Register(RegisterType::D),
        );
        instructions[0xB3] = Instruction::instruction_1_operand(
            "OR E",
            InstructionType::OR,
            Operand::Register(RegisterType::E),
        );
        instructions[0xB4] = Instruction::instruction_1_operand(
            "OR H",
            InstructionType::OR,
            Operand::Register(RegisterType::H),
        );
        instructions[0xB5] = Instruction::instruction_1_operand(
            "OR L",
            InstructionType::OR,
            Operand::Register(RegisterType::L),
        );
        instructions[0xB6] = Instruction::instruction_1_operand(
            "OR (HL)",
            InstructionType::OR,
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0xB7] = Instruction::instruction_1_operand(
            "OR A",
            InstructionType::OR,
            Operand::Register(RegisterType::A),
        );

        instructions[0xB8] = Instruction::instruction_1_operand(
            "CP B",
            InstructionType::CP,
            Operand::Register(RegisterType::B),
        );
        instructions[0xB9] = Instruction::instruction_1_operand(
            "CP C",
            InstructionType::CP,
            Operand::Register(RegisterType::C),
        );
        instructions[0xBA] = Instruction::instruction_1_operand(
            "CP D",
            InstructionType::CP,
            Operand::Register(RegisterType::D),
        );
        instructions[0xBB] = Instruction::instruction_1_operand(
            "CP E",
            InstructionType::CP,
            Operand::Register(RegisterType::E),
        );
        instructions[0xBC] = Instruction::instruction_1_operand(
            "CP H",
            InstructionType::CP,
            Operand::Register(RegisterType::H),
        );
        instructions[0xBD] = Instruction::instruction_1_operand(
            "CP L",
            InstructionType::CP,
            Operand::Register(RegisterType::L),
        );
        instructions[0xBE] = Instruction::instruction_1_operand(
            "CP (HL)",
            InstructionType::CP,
            Operand::Indirect(RegisterType::HL),
        );
        instructions[0xBF] = Instruction::instruction_1_operand(
            "CP A",
            InstructionType::CP,
            Operand::Register(RegisterType::A),
        );

        instructions[0xC0] = Instruction::instruction_no_operands_with_condition(
            "RET NZ",
            InstructionType::RET,
            ConditionType::NZ,
        );

        instructions[0xC1] = Instruction::instruction_1_operand(
            "POP BC",
            InstructionType::POP,
            Operand::Register(RegisterType::BC),
        );

        instructions[0xC2] = Instruction::instruction_1_operand_with_condition(
            "JP NZ,a16",
            InstructionType::JP,
            Operand::A16,
            ConditionType::NZ,
        );
        instructions[0xC3] =
            Instruction::instruction_1_operand("JP a16", InstructionType::JP, Operand::A16);

        instructions[0xC4] = Instruction::instruction_1_operand_with_condition(
            "CALL NZ,a16",
            InstructionType::CALL,
            Operand::A16,
            ConditionType::NZ,
        );

        instructions[0xC5] = Instruction::instruction_1_operand(
            "PUSH BC",
            InstructionType::PUSH,
            Operand::Register(RegisterType::BC),
        );

        instructions[0xC6] = Instruction::instruction_2_operands(
            "ADD A,d8",
            InstructionType::ADD,
            Operand::Register(RegisterType::A),
            Operand::D8,
        );

        instructions[0xC7] =
            Instruction::instruction_with_parameter("RST 00H", InstructionType::RST, 0x00);

        instructions[0xC8] = Instruction::instruction_no_operands_with_condition(
            "RET Z",
            InstructionType::RET,
            ConditionType::Z,
        );

        instructions[0xC9] = Instruction::instruction_no_operands("RET", InstructionType::RET);

        instructions[0xCA] = Instruction::instruction_1_operand_with_condition(
            "JP Z,a16",
            InstructionType::JP,
            Operand::A16,
            ConditionType::Z,
        );

        instructions[0xCB] =
            Instruction::instruction_1_operand("LD B,(HL)", InstructionType::CB, Operand::D8);

        instructions[0xCC] = Instruction::instruction_1_operand_with_condition(
            "CB d8",
            InstructionType::CALL,
            Operand::A16,
            ConditionType::Z,
        );

        instructions[0xCD] =
            Instruction::instruction_1_operand("CALL a16", InstructionType::CALL, Operand::A16);

        instructions[0xCE] = Instruction::instruction_2_operands(
            "ADC A,d8",
            InstructionType::ADC,
            Operand::Register(RegisterType::A),
            Operand::D8,
        );
        instructions[0xCF] =
            Instruction::instruction_with_parameter("RST 08H", InstructionType::RST, 0x08);
        instructions[0xD0] = Instruction::instruction_no_operands_with_condition(
            "RET NC",
            InstructionType::RET,
            ConditionType::NC,
        );

        instructions[0xD1] = Instruction::instruction_1_operand(
            "POP DE",
            InstructionType::POP,
            Operand::Register(RegisterType::DE),
        );

        instructions[0xD2] = Instruction::instruction_1_operand_with_condition(
            "JP NC,a16",
            InstructionType::JP,
            Operand::A16,
            ConditionType::NC,
        );
        instructions[0xD4] = Instruction::instruction_1_operand_with_condition(
            "CALL NC,a16",
            InstructionType::CALL,
            Operand::A16,
            ConditionType::NC,
        );
        instructions[0xD5] = Instruction::instruction_1_operand(
            "PUSH DE",
            InstructionType::PUSH,
            Operand::Register(RegisterType::DE),
        );

        instructions[0xD6] =
            Instruction::instruction_1_operand("SUB d8", InstructionType::SUB, Operand::D8);
        instructions[0xD7] =
            Instruction::instruction_with_parameter("RST 10H", InstructionType::RST, 0x10);

        instructions[0xD8] = Instruction::instruction_no_operands_with_condition(
            "RET C",
            InstructionType::RET,
            ConditionType::C,
        );

        instructions[0xD9] = Instruction::instruction_no_operands("RETI", InstructionType::RETI);
        instructions[0xDA] = Instruction::instruction_1_operand_with_condition(
            "JP C,a16",
            InstructionType::JP,
            Operand::A16,
            ConditionType::C,
        );
        instructions[0xDC] = Instruction::instruction_1_operand_with_condition(
            "CALL C,a16",
            InstructionType::CALL,
            Operand::A16,
            ConditionType::C,
        );
        instructions[0xDE] = Instruction::instruction_2_operands(
            "SBC A d8",
            InstructionType::SBC,
            Operand::Register(RegisterType::A),
            Operand::D8,
        );
        instructions[0xDF] =
            Instruction::instruction_with_parameter("RST 18H", InstructionType::RST, 0x18);
        instructions[0xE0] = Instruction::instruction_2_operands(
            "LDH (a8),A",
            InstructionType::LDH,
            Operand::A8Indirect,
            Operand::Register(RegisterType::A),
        );

        instructions[0xE1] = Instruction::instruction_1_operand(
            "POP HL",
            InstructionType::POP,
            Operand::Register(RegisterType::HL),
        );

        instructions[0xE2] = Instruction::instruction_2_operands(
            "LD (C),A",
            InstructionType::LD,
            Operand::Indirect(RegisterType::C),
            Operand::Register(RegisterType::A),
        );
        instructions[0xE5] = Instruction::instruction_1_operand(
            "PUSH HL",
            InstructionType::PUSH,
            Operand::Register(RegisterType::HL),
        );

        instructions[0xE6] =
            Instruction::instruction_1_operand("AND d8", InstructionType::AND, Operand::D8);

        instructions[0xE7] =
            Instruction::instruction_with_parameter("RST 20H", InstructionType::RST, 0x20);

        instructions[0xE8] = Instruction::instruction_2_operands(
            "ADD SP,r8",
            InstructionType::ADD,
            Operand::Register(RegisterType::SP),
            Operand::R8,
        );
        instructions[0xE9] = Instruction::instruction_1_operand(
            "JP HL",
            InstructionType::JP,
            Operand::Register(RegisterType::HL),
        );
        instructions[0xEA] = Instruction::instruction_2_operands(
            "LD (a16),A",
            InstructionType::LD,
            Operand::A16Indirect,
            Operand::Register(RegisterType::A),
        );

        instructions[0xEE] =
            Instruction::instruction_1_operand("XOR d8", InstructionType::XOR, Operand::D8);

        instructions[0xEF] =
            Instruction::instruction_with_parameter("RST 28H", InstructionType::RST, 0x28);

        instructions[0xF0] = Instruction::instruction_2_operands(
            "LDH A,(a8)",
            InstructionType::LDH,
            Operand::Register(RegisterType::A),
            Operand::A8Indirect,
        );

        instructions[0xF1] = Instruction::instruction_1_operand(
            "POP AF",
            InstructionType::POP,
            Operand::Register(RegisterType::AF),
        );

        instructions[0xF2] = Instruction::instruction_2_operands(
            "LD A,(C)",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::Indirect(RegisterType::C),
        );

        instructions[0xF3] = Instruction::instruction_no_operands("DI", InstructionType::DI);

        instructions[0xF5] = Instruction::instruction_1_operand(
            "PUSH AF",
            InstructionType::PUSH,
            Operand::Register(RegisterType::AF),
        );
        instructions[0xF6] =
            Instruction::instruction_1_operand("OR d8", InstructionType::OR, Operand::D8);

        instructions[0xF7] =
            Instruction::instruction_with_parameter("RST 30H", InstructionType::RST, 0x30);

        instructions[0xF8] = Instruction::instruction_2_operands(
            "LD HL,SP+r8",
            InstructionType::LD,
            Operand::Register(RegisterType::HL),
            Operand::SpPlusR8,
        );
        instructions[0xF9] = Instruction::instruction_2_operands(
            "LD SP,HL",
            InstructionType::LD,
            Operand::Register(RegisterType::SP),
            Operand::Register(RegisterType::HL),
        );
        instructions[0xFA] = Instruction::instruction_2_operands(
            "LD A,(a16)",
            InstructionType::LD,
            Operand::Register(RegisterType::A),
            Operand::A16Indirect,
        );

        instructions[0xFB] = Instruction::instruction_no_operands("EI", InstructionType::EI);
        instructions[0xFE] =
            Instruction::instruction_1_operand("CP d8", InstructionType::CP, Operand::D8);

        instructions[0xFF] =
            Instruction::instruction_with_parameter("RST 38H", InstructionType::RST, 0x38);
        Self { instructions }
    }

    pub fn get_instruction_by_opcode(&self, opcode: u8) -> &Instruction {
        &self.instructions[opcode as usize]
    }
}
