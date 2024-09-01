mod common;

use common::util::TestBus;
use rusty_gb::cpu::CpuContext;

#[test]
fn test_rra() {
    let bus = TestBus::create(vec![0x1F, 0x1F, 0x00, 0x01]);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;
    cpu.cpu_registers.a = 0xA;
    cpu.cpu_registers.f.register = 0x00;
    cpu.cpu_registers.pc = 0x0000;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.a, 0x5);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.cpu_registers.pc, 1);
    assert_eq!(cpu.ticks, 4);

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.a, 0x2);
    assert_eq!(cpu.cpu_registers.f.register, 0x10);
    assert_eq!(cpu.cpu_registers.pc, 2);
    assert_eq!(cpu.ticks, 8);
}
