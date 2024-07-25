use super::{
    alu_operations::{
        AddOperation, AddOperation16, AddRelativeOperation, AddWithCarryOperation, AluOperation,
        AndOperation, CpOperation, DecOperation, DecOperation16, IncOperation, IncOperation16,
        OrOperation, RraOperation, SubOperation, XorOperation,
    },
    RegisterType,
};

#[derive(Debug, Clone, Copy)]
pub struct ExecutionPlan {
    fetch_action: FetchAction,
    arithmetic_logic_unit_action: ArithmeticLogicUnitAction,
    custom_action: CustomAction,
    store_action: StoreAction,
}

impl ExecutionPlan {
    pub fn new(
        fetch_action: FetchAction,
        arithmetic_logic_unit_action: ArithmeticLogicUnitAction,
        store_action: StoreAction,
    ) -> Self {
        Self {
            fetch_action,
            arithmetic_logic_unit_action,
            custom_action: CustomAction::None,
            store_action,
        }
    }

    pub fn with_custom_action(
        fetch_action: FetchAction,
        arithmetic_logic_unit_action: ArithmeticLogicUnitAction,
        custom_action: CustomAction,
        store_action: StoreAction,
    ) -> Self {
        Self {
            fetch_action,
            arithmetic_logic_unit_action,
            custom_action,
            store_action,
        }
    }
    pub fn get_fetch_action(&self) -> &FetchAction {
        &self.fetch_action
    }

    pub fn get_arithmetic_logic_unit_action(&self) -> &ArithmeticLogicUnitAction {
        &self.arithmetic_logic_unit_action
    }

    pub fn get_store_action(&self) -> &StoreAction {
        &self.store_action
    }
}

impl Default for ExecutionPlan {
    fn default() -> Self {
        Self {
            fetch_action: FetchAction::None,
            arithmetic_logic_unit_action: ArithmeticLogicUnitAction::None,
            custom_action: CustomAction::None,
            store_action: StoreAction::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FetchAction {
    // No action
    None,
    // Fetch actions
    FetchData, // Fetch 8 bits of data that is currently pointed to by the program counter - PC
    FetchSignedData, // Fetch 8 bits of signed data that is currently pointed to by the program
    // counter - PC
    FetchData16Bits, // Fetch 16 bits of data that is currently pointed to by the program counter
    // In this case we should read the current PC and PC + 1
    FetchAddress, // Fetch the address that is currently pointed to by the program counter
    FetchAddressZeroPage, // Fetch the address that is currently pointed to by the program counter and
    // get the data from the zero page 0xFF + address from the memory bus
    FetchRegister(RegisterType), // Fetch the value of a register A, B, C, D, E, H or L
    FetchRegister16Bits(RegisterType), // Fetch the value of a 16-bit register
    FetchRegister16BitsWithOffset(RegisterType), // Fetch the value of a 16-bit register with
    // an 8-bit offset (signed) value
    FetchIndirect(RegisterType), // Fetch the value from the memory address pointed by a 16-bit register
    FetchIndirectZeroPage(RegisterType), // Fetch the value from the memory address pointed by
    // a 0xFF00 + 8-bit register. Only used in the C register
    FetchIndirectAndIncrement(RegisterType), // Fetch the value from the memory address pointed by
    // a 16-bit register and increment the register
    // This is only used in the HL register
    FetchIndirectAndDecrement(RegisterType), // Fetch the value from the memory address pointed by
    // a 16-bit register and decrement the register
    // This is only used in the HL register
    FetchStack, // Fetch the value from the stack --> POP
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArithmeticLogicUnitAction {
    None,
    Inc,                       // Increment the value of a register A, B, C, D, E, H or L
    Inc16,                     // Increment the value of a 16-bit register BC, DE, HL or SP
    Dec,                       // Decrement the value of a register A, B, C, D, E, H or L
    Dec16,                     // Decrement the value of a 16-bit register BC, DE, HL or SP
    Add(RegisterType),         // Add the value of a register A, B, C, D, E, H or L
    Add16(RegisterType),       // Add the value of a 16-bit register BC, DE, HL or SP
    AddRelative(RegisterType), // Add the value of a 16-bit register with an 8-bit offset
    // (signed)
    AddWithCarry(RegisterType), // Add the value of a register A, B, C, D, E, H or L
    Sub(RegisterType),          // Subtract the value of a register A, B, C, D, E, H or L
    SubWithCarry(RegisterType), // Subtract the value of a register A, B, C, D, E, H or L with
    // carry

    // Bitwise operations
    Rra, // Rotate the value of register A to the right through the carry flag
    And, // Logical AND the value of a register A, B, C, D, E, H or L with register A
    Xor, // Logical XOR the value of a register A, B, C, D, E, H or L with register A
    Or,  // Logical OR the value of a register A, B, C, D, E, H or L with register A
    Cp,  // Compare the value of a register A, B, C, D, E, H or L with register A
}

impl ArithmeticLogicUnitAction {
    pub fn get_operation(&self) -> Box<dyn AluOperation + 'static> {
        match self {
            ArithmeticLogicUnitAction::None => unimplemented!("ALU None not implemented"),
            ArithmeticLogicUnitAction::Inc => Box::new(IncOperation::new()),
            ArithmeticLogicUnitAction::Inc16 => Box::new(IncOperation16::new()),
            ArithmeticLogicUnitAction::Dec => Box::new(DecOperation::new()),
            ArithmeticLogicUnitAction::Dec16 => Box::new(DecOperation16::new()),
            ArithmeticLogicUnitAction::Add(register_type) => {
                Box::new(AddOperation::new(register_type))
            }
            ArithmeticLogicUnitAction::Add16(register_type) => {
                Box::new(AddOperation16::new(register_type))
            }
            ArithmeticLogicUnitAction::AddRelative(register_type) => {
                Box::new(AddRelativeOperation::new(register_type))
            }
            ArithmeticLogicUnitAction::AddWithCarry(register_type) => {
                Box::new(AddWithCarryOperation::new(register_type))
            }
            ArithmeticLogicUnitAction::Sub(register_type) => {
                Box::new(SubOperation::new(register_type))
            }
            ArithmeticLogicUnitAction::SubWithCarry(_) => todo!(),
            ArithmeticLogicUnitAction::Rra => Box::new(RraOperation::new()),
            ArithmeticLogicUnitAction::And => Box::new(AndOperation::new()),
            ArithmeticLogicUnitAction::Xor => Box::new(XorOperation::new()),
            ArithmeticLogicUnitAction::Or => Box::new(OrOperation::new()),
            ArithmeticLogicUnitAction::Cp => Box::new(CpOperation::new()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CustomAction{
    None,
    PrefixCB,
}


#[derive(Debug, Clone, Copy)]
pub enum StoreAction {
    None,
    StoreRegister(RegisterType), // Store the result to an 8-bit register: A, B, C, D, E, H or L
    StoreRegister16Bits(RegisterType), // Store the result to a 16-bit register: AF, BC, DE, HL, SP, PC
    StoreStack,                        // Store the result to the stack --> PUSH
    StoreIndirect(RegisterType),       // Store the result to the memory address pointed by a
    // 16-bit register
    StoreIndirectZeroPage(RegisterType), // Store the result to the memory address pointed by
    // a 0xFF00 + 8-bit register. Only used in the C register
    StoreIndirectAndIncrement(RegisterType), // Store the result to the memory address pointed by
    // a 16-bit register and increment the register
    // This is only used in the HL register
    StoreIndirectAndDecrement(RegisterType), // Store the result to the memory address pointed by
    // a 16-bit register and decrement the register
    // This is only used in the HL register
    StoreAddress,       // Store the result to a memory address
    StoreAddress16Bits, // Store 16 bits of data to a memory address and address + 1
    StoreAddressZeroPage, // Store the result to a memory address in the zero page (LDH for
                        // example)
}
