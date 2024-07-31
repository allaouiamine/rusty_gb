mod common;

use common::util::TestBus;
use rusty_gb::cpu::{CpuContext, RegisterType};

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
    assert_eq!(cpu.ticks, 8);
}

#[test]
fn test_cb_hl() {
    let rom = vec![
        0xCB, 0x3E, // SRL (HL)
        0xFD, 0xFF,
    ];
    let bus = TestBus::create(rom);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers.set_register_16(&RegisterType::HL, 0x0003);
    cpu.cpu_registers.pc = 0;
    cpu.cpu_registers.f.register = 0x00;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(
        cpu.bus
            .lock()
            .unwrap()
            .bus_read(cpu.cpu_registers.get_register_16(&RegisterType::HL)),
        0x7F
    );
    assert_eq!(cpu.cpu_registers.f.register, 0x10); // C=1
    assert_eq!(cpu.ticks, 16);
}
