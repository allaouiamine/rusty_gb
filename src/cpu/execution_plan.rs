use super::instruction::{InstructionType, RegisterType};

#[derive(Default)]
pub struct ExecutionPlan {
    pub actions: Vec<Action>,
}


pub enum Action {
    // No action
    None,
    // Fetch actions
    FetchData, // Fetch 8 bits of data that is currently pointed to by the program counter - PC
    FetchData16Bits, // Fetch 16 bits of data that is currently pointed to by the program counter
                     // In this case we should read the current PC and PC + 1
    FetchRegister(RegisterType), // Fetch the value of a register A, B, C, D, E, H or L
    FetchRegister16Bits(RegisterType), // Fetch the value of a 16-bit register
    FetchRegister16BitsWithOffset(RegisterType), // Fetch the value of a 16-bit register with
                                                     // an 8-bit offset (signed) value
                                                     // This is only used in the stack pointer SP
    FetchIndirect(RegisterType), // Fetch the value from the memory address pointed by a 16-bit register
    FetchIndirectZeroPage(RegisterType), // Fetch the value from the memory address pointed by
                                         // a 0xFF00 + 8-bit register. Only used in the C register
    FetchIndirectAndIncremment(RegisterType), // Fetch the value from the memory address pointed by
                                              // a 16-bit register and increment the register
                                              // This is only used in the HL register
    FetchIndirectAndDecrement(RegisterType), // Fetch the value from the memory address pointed by
                                             // a 16-bit register and decrement the register 
                                             // This is only used in the HL register


    // Arithmetic Actions
    // TODO: Use a closure to perform the arithmetic operation
    // INC, DEC, ADD, ADC, SUB, SBC, AND, OR, XOR, CP
    ArithmeticOperation(InstructionType), // Process the instruction
    ArithmeticOperation16Bits(InstructionType), // Process the instruction


    // Store actions
    StoreRegister(RegisterType), // Store the result to an 8-bit register: A, B, C, D, E, H or L
    StoreRegister16Bits(RegisterType), // Store the result to a 16-bit register: AF, BC, DE, HL, SP, PC
    StoreRegisterIndirect(RegisterType), // Store the result to the memory address pointed by a
                                         // 16-bit register
    StoreIndirectZeroPage(RegisterType), // Store the result to the memory address pointed by
                                         // a 0xFF00 + 8-bit register. Only used in the C register
    StoreIndirectAndIncrement(RegisterType), // Store the result to the memory address pointed by
                                             // a 16-bit register and increment the register
                                             // This is only used in the HL register
    StoreIndirectAndDecrement(RegisterType), // Store the result to the memory address pointed by
                                             // a 16-bit register and decrement the register
                                             // This is only used in the HL register
    StoreAddress, // Store the result to a memory address
}

