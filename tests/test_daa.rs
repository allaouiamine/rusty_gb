mod common;

use common::util::TestBus;
use rusty_gb::cpu::{
    registers::{CpuRegisters, FlagsRegister},
    CpuContext, RegisterType,
};

#[test]
fn test_daa() {
    let bus = TestBus::create(vec![0x27, 0x27, 0x27, 0x27, 0x27, 0x27, 0xFD, 0xFD]);
    let mut cpu = CpuContext::new(bus);

    cpu.cpu_registers.pc = 0x00;
    cpu.cpu_registers.a = 0x00;
    cpu.cpu_registers.f.register = 0x00;
    let _ = cpu.cpu_step();
    assert_eq!(cpu.cpu_registers.a, 0x00);
    assert_eq!(cpu.cpu_registers.f.register, 0x80);
    assert_eq!(cpu.ticks, 4);

    cpu.cpu_registers.a = 0x13;
    cpu.cpu_registers.f.register = 0x20; // H=1
    cpu.ticks = 0;
    let _ = cpu.cpu_step();
    assert_eq!(cpu.cpu_registers.a, 0x19);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.ticks, 4);

    cpu.cpu_registers.a = 0x1A;
    cpu.cpu_registers.f.register = 0x00;
    cpu.ticks = 0;
    let _ = cpu.cpu_step();
    assert_eq!(cpu.cpu_registers.a, 0x20);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.ticks, 4);

    cpu.cpu_registers.a = 0x13;
    cpu.cpu_registers.f.register = 0x00;
    cpu.ticks = 0;
    let _ = cpu.cpu_step();
    assert_eq!(cpu.cpu_registers.a, 0x13);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.ticks, 4);

    cpu.cpu_registers.a = 0x13;
    cpu.cpu_registers.f.register = 0x10; // C=1
    cpu.ticks = 0;
    let _ = cpu.cpu_step();
    assert_eq!(cpu.cpu_registers.a, 0x73);
    assert_eq!(cpu.cpu_registers.f.register, 0x10);
    assert_eq!(cpu.ticks, 4);

    cpu.cpu_registers.a = 0xA0; // 100
    cpu.cpu_registers.f.register = 0x10;
    cpu.ticks = 0;
    let _ = cpu.cpu_step();
    assert_eq!(cpu.cpu_registers.a, 0x00);
    assert_eq!(cpu.cpu_registers.f.register, 0x90);
    assert_eq!(cpu.ticks, 4);
}
