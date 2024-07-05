mod common;

use common::util::TestBus;
use rusty_gb::cpu::{registers::{CpuRegisters, FlagsRegister}, CpuContext, RegisterType};

#[test]
fn test_or() {
    let bus = TestBus::create(vec![0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xF6, 0x80, 0x20]);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers = CpuRegisters{
        a: 0,
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
    let _ = cpu.cpu_step().unwrap(); // OR B
    assert_eq!(cpu.cpu_registers.a, 0x00);
    assert_eq!(cpu.cpu_registers.f.register, 0x80);
    assert_eq!(cpu.cpu_registers.pc, 1);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // OR C
    assert_eq!(cpu.cpu_registers.a, 0x01);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.cpu_registers.pc, 2);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // OR D
    assert_eq!(cpu.cpu_registers.a, 0x03);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.cpu_registers.pc, 3);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // OR E
    assert_eq!(cpu.cpu_registers.a, 0x07);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // OR H
    assert_eq!(cpu.cpu_registers.a, 0x0F);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.cpu_registers.pc, 5);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // OR L
    assert_eq!(cpu.cpu_registers.a, 0x1F);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.ticks, 4);

    cpu.cpu_registers.set_register_16(&RegisterType::HL, 0xA);
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // OR (HL) HL = 0x20
    assert_eq!(cpu.cpu_registers.a, 0x3F);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.cpu_registers.pc, 7);
    assert_eq!(cpu.ticks, 8);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // OR A --> no change
    assert_eq!(cpu.cpu_registers.a, 0x3F);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.cpu_registers.pc, 8);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap(); // OR d8 = 0x80
    assert_eq!(cpu.cpu_registers.a, 0xBF);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.cpu_registers.pc, 10);
    assert_eq!(cpu.ticks, 8);
}
