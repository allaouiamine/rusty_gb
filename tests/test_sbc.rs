mod common;

use common::util::TestBus;
use rusty_gb::cpu::CpuContext;

#[test]
fn test_sbc_carry() {
    let rom = vec![
        0xDE, 0xFF, // SBC A, 0xFF
        0xFD,
    ];
    let bus = TestBus::create(rom);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0;
    cpu.cpu_registers.a = 0x0000;
    cpu.cpu_registers.f.register = 0x10;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.a, 0x00);
    assert_eq!(cpu.cpu_registers.f.register, 0xF0);
    assert_eq!(cpu.ticks, 8);
}
