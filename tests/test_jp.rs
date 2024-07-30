mod common;

use common::util::TestBus;
use rusty_gb::cpu::CpuContext;

#[test]
fn test_jp_nz_ok() {
    let bus = TestBus::create(vec![
        0xC2, 0x22, 0x33, // JP NZ 0x3322
        0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.f.register = 0x00; // Z = 0
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 16);
}

#[test]
fn test_jp_nz_nok() {
    let bus = TestBus::create(vec![
        0xC2, 0x22, 0x33, // JP NZ 0x3322
        0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.f.register = 0x80; // Z = 1
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 12);
}

#[test]
fn test_jp_z_ok() {
    let bus = TestBus::create(vec![
        0xCA, 0x22, 0x33, // JP Z 0x3322
        0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.f.register = 0x80; // Z = 1
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 16);
}

#[test]
fn test_jp_z_nok() {
    let bus = TestBus::create(vec![
        0xCA, 0x22, 0x33, // JP NZ 0x3322
        0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.f.register = 0x00; // Z = 0
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 12);
}

#[test]
fn test_jp_nc_ok() {
    let bus = TestBus::create(vec![
        0xD2, 0x22, 0x33, // JP NZ 0x3322
        0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.f.register = 0x00; // C = 0
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 16);
}

#[test]
fn test_jp_nc_nok() {
    let bus = TestBus::create(vec![
        0xD2, 0x22, 0x33, // JP NC 0x3322
        0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.f.register = 0x10; // C = 1
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 12);
}

#[test]
fn test_jp_c_ok() {
    let bus = TestBus::create(vec![
        0xDA, 0x22, 0x33, // JP C 0x3322
        0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.f.register = 0x10; // C = 1
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 16);
}

#[test]
fn test_jp_c_nok() {
    let bus = TestBus::create(vec![
        0xDA, 0x22, 0x33, // JP NZ 0x3322
        0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.f.register = 0x00; // C = 0
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 12);
}

#[test]
fn test_jp() {
    let bus = TestBus::create(vec![
        0xC3, 0x22, 0x33, // JP 0x3322
        0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 16);
}

#[test]
fn test_jp_hl() {
    let bus = TestBus::create(vec![
        0xE9, // JP (HL)
        0xFD, 0xFD,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.h = 0x33;
    cpu.cpu_registers.l = 0x22;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
    // TODO: Fix the emu cycles. It should be 12 not 8
    //assert_eq!(cpu.ticks, 4);
}
