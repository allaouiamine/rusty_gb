mod common;

use common::util::TestBus;
use rusty_gb::cpu::{registers::{CpuRegisters, FlagsRegister}, CpuContext, RegisterType};

#[test]
fn test_cp() {
    let bus = TestBus::create(vec![0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE, 0xBF, 0xFE, 0x80, 0x20]);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers = CpuRegisters{
        a: 0x3,
        f: FlagsRegister {register: 0},
        b: 0,
        c: 0x1,
        d: 0x2,
        e: 0x4,
        h: 0x8,
        l: 0x10,
        pc: 0,
        sp: 0,
    };
    let _ = cpu.cpu_step().unwrap(); // CP B
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0x40);
    assert_eq!(cpu.cpu_registers.pc, 1);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // CP C
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0x40);
    assert_eq!(cpu.cpu_registers.pc, 2);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // CP D
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0x40);
    assert_eq!(cpu.cpu_registers.pc, 3);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // CP E
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0x70);
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // CP H
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0x70);
    assert_eq!(cpu.cpu_registers.pc, 5);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // CP L
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0x50);
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.ticks, 4);

    cpu.cpu_registers.set_register_16(&RegisterType::HL, 0xA);
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // CP (HL) HL = 0x20
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0x50);
    assert_eq!(cpu.cpu_registers.pc, 7);
    assert_eq!(cpu.ticks, 8);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // CP A --> no change
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0xC0);
    assert_eq!(cpu.cpu_registers.pc, 8);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // CP d8 = 0x80
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0x50);
    assert_eq!(cpu.cpu_registers.pc, 10);
    assert_eq!(cpu.ticks, 8);
}
