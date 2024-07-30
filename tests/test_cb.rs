mod common;

use common::util::TestBus;
use rusty_gb::cpu::CpuContext;

#[test]
fn test_cb() {
    let rom = vec![
        0xCB, 0x38, // SRL B
        0xFD,
    ];
    let bus = TestBus::create(rom);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers.b = 0xFF;
    cpu.cpu_registers.pc = 0;
    cpu.cpu_registers.f.register = 0x00;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.b, 0x7F);
    assert_eq!(cpu.cpu_registers.f.register, 0x10); // C=1
    assert_eq!(cpu.ticks, 12);
}
