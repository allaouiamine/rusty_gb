mod common;

use common::util::TestBus;
use rusty_gb::cpu::{
    registers::{CpuRegisters, FlagsRegister},
    CpuContext, RegisterType,
};

#[test]
fn test_push_pop() {
    let bus = TestBus::create(vec![
        0xC5, // PUSH BC
        0xD5, // PUSH DE
        0xE5, // PUSH HL
        0xF5, // PUSH AF
        0xC1, // POP BC
        0xD1, // POP DE
        0xE1, // POP HL
        0xF1, // POP AF
        //
        //stack
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // <- SP = 11
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers = CpuRegisters {
        a: 0x01,
        f: FlagsRegister { register: 0x20 },
        b: 0x07,
        c: 0x08,
        d: 0x05,
        e: 0x06,
        h: 0x03,
        l: 0x04,
        sp: 0x10,
        pc: 0x00,
    };

    // PUSH
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 12);
    assert_eq!(cpu.cpu_registers.sp, 0x0E);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::BC), 0x0708);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::DE), 0x0506);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x0304);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::AF), 0x0120); // the lower 4 bits
                                                                              // of F are always 0

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 24);
    assert_eq!(cpu.cpu_registers.sp, 0x0C);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::BC), 0x0708);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::DE), 0x0506);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x0304);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::AF), 0x0120);

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 36);
    assert_eq!(cpu.cpu_registers.sp, 0x0A);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::BC), 0x0708);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::DE), 0x0506);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x0304);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::AF), 0x0120);

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 48);
    assert_eq!(cpu.cpu_registers.sp, 0x08);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::BC), 0x0708);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::DE), 0x0506);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x0304);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::AF), 0x0120);

    // POP
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 60);
    assert_eq!(cpu.cpu_registers.sp, 0x0A);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::BC), 0x0120);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::DE), 0x0506);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x0304);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::AF), 0x0120);

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 72);
    assert_eq!(cpu.cpu_registers.sp, 0x0C);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::BC), 0x0120);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::DE), 0x0304);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x0304);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::AF), 0x0120);

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 84);
    assert_eq!(cpu.cpu_registers.sp, 0x0E);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::BC), 0x0120);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::DE), 0x0304);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x0506);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::AF), 0x0120);

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 96);
    assert_eq!(cpu.cpu_registers.sp, 0x10);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::BC), 0x0120);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::DE), 0x0304);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x0506);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::AF), 0x0700); // the lower 4 bits
                                                                              // of F are always 0
}
