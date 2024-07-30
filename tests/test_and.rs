mod common;

use common::util::TestBus;
use rusty_gb::cpu::{
    registers::{CpuRegisters, FlagsRegister},
    CpuContext, RegisterType,
};

#[test]
fn test_and() {
    let bus = TestBus::create(vec![
        0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xE6, 0x81, 0x20,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers = CpuRegisters {
        a: 0x1A,
        f: FlagsRegister { register: 0 },
        b: 0,
        c: 0x1,
        d: 0x2,
        e: 0x4,
        h: 0x8,
        l: 0x10,
        pc: 0,
        sp: 0,
    };
    let _ = cpu.cpu_step().unwrap(); // AND B
    assert_eq!(cpu.cpu_registers.a, 0x00);
    assert_eq!(cpu.cpu_registers.f.register, 0xA0);
    assert_eq!(cpu.cpu_registers.pc, 1);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    cpu.cpu_registers.a = 0x1B;
    let _ = cpu.cpu_step().unwrap(); // AND C
    assert_eq!(cpu.cpu_registers.a, 0x01);
    assert_eq!(cpu.cpu_registers.f.register, 0x20);
    assert_eq!(cpu.cpu_registers.pc, 2);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    cpu.cpu_registers.a = 0x1B;
    let _ = cpu.cpu_step().unwrap(); // AND D
    assert_eq!(cpu.cpu_registers.a, 0x02);
    assert_eq!(cpu.cpu_registers.f.register, 0x20);
    assert_eq!(cpu.cpu_registers.pc, 3);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    cpu.cpu_registers.a = 0x1B;
    let _ = cpu.cpu_step().unwrap(); // AND E
    assert_eq!(cpu.cpu_registers.a, 0x00);
    assert_eq!(cpu.cpu_registers.f.register, 0xA0);
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    cpu.cpu_registers.a = 0x1B;
    let _ = cpu.cpu_step().unwrap(); // AND H
    assert_eq!(cpu.cpu_registers.a, 0x08);
    assert_eq!(cpu.cpu_registers.f.register, 0x20);
    assert_eq!(cpu.cpu_registers.pc, 5);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    cpu.cpu_registers.a = 0x1B;
    let _ = cpu.cpu_step().unwrap(); // AND L
    assert_eq!(cpu.cpu_registers.a, 0x10);
    assert_eq!(cpu.cpu_registers.f.register, 0x20);
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.ticks, 4);

    cpu.cpu_registers.set_register_16(&RegisterType::HL, 0xA);
    cpu.ticks = 0;
    cpu.cpu_registers.a = 0x1F;
    let _ = cpu.cpu_step().unwrap(); // AND (HL)
    assert_eq!(cpu.cpu_registers.a, 0x00);
    assert_eq!(cpu.cpu_registers.f.register, 0xA0);
    assert_eq!(cpu.cpu_registers.pc, 7);
    assert_eq!(cpu.ticks, 8);

    cpu.ticks = 0;
    cpu.cpu_registers.a = 0x1B;
    let _ = cpu.cpu_step().unwrap(); // AND A
    assert_eq!(cpu.cpu_registers.a, 0x1B);
    assert_eq!(cpu.cpu_registers.f.register, 0x20);
    assert_eq!(cpu.cpu_registers.pc, 8);
    assert_eq!(cpu.ticks, 4);

    cpu.ticks = 0;
    cpu.cpu_registers.a = 0x1B;
    let _ = cpu.cpu_step().unwrap(); // AND 0x81
    assert_eq!(cpu.cpu_registers.a, 0x01);
    assert_eq!(cpu.cpu_registers.f.register, 0x20);
    assert_eq!(cpu.cpu_registers.pc, 10);
    assert_eq!(cpu.ticks, 8);
}
