mod common;

use common::util::TestBus;
use rusty_gb::cpu::CpuContext;

#[test]
fn test_add_sp_r8() {
    let mut bus_data = vec![0xFD; 0x10000];
    let rom = vec![
        0xE8, 0x01, // ADD SP, 1
        0xFD,
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0;
    cpu.cpu_registers.sp = 0x0000;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.sp, 0x1);
    assert_eq!(cpu.ticks, 16);
}

#[test]
fn test_add_sp_r8_rotate_back() {
    let mut bus_data = vec![0xFD; 0x10000];
    let rom = vec![
        0xE8, 0xFF, // ADD SP, 0xFF (-1)
        0xFD,
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0;
    cpu.cpu_registers.sp = 0x0000;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.sp, 0xFFFF);
    assert_eq!(cpu.cpu_registers.f.register, 0x00);
    assert_eq!(cpu.ticks, 16);
}

#[test]
fn test_add_sp_r8_carry() {
    let mut bus_data = vec![0xFD; 0x10000];
    let rom = vec![
        0xE8, 0x01, // ADD SP, 0x01
        0xFD,
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0;
    cpu.cpu_registers.sp = 0x00FF;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.sp, 0x0100);
    assert_eq!(cpu.cpu_registers.f.register, 0x30);
    assert_eq!(cpu.ticks, 16);
}

#[test]
fn test_add_sp_r8_negative() {
    let mut bus_data = vec![0xFD; 0x10000];
    let rom = vec![
        0xE8, 0x88, // ADD SP, 0x88 --> signed equivalent is -120 (-0x78)
        0xFD,
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x00;
    cpu.cpu_registers.sp = 0x0099;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.sp, 0x21);
    assert_eq!(cpu.ticks, 16);
}
