mod common;

use common::util::TestBus;
use rusty_gb::cpu::CpuContext;

#[test]
fn test_jr() {
    let bus = TestBus::create(vec![
        0x18, 0x00, // JR 0x00 --> DO NOTHING
        0x18, 0x02, // JR 0x02
        0x00, 0x00, // ignore
        0xFD, // Jump here!
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 2);
    // TODO: Fix the emu cycles. It should be 12 not 8
    // assert_eq!(cpu.ticks, 12);
    //
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.bus_read(cpu.cpu_registers.pc), 0xFD);
}

#[test]
fn test_jr_nz() {
    let bus = TestBus::create(vec![
        0xFD, 0xDD, // Should jump here if NZ
        0x20, 0xFC, // Start here JR NZ 0xFR (which is -4)
        0xED, // move here if NZ condition is not satisfied
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0002;
    cpu.cpu_registers.f.register = 0x80; // Z = 1
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4); // the NZ condition is not met. So there is no jump
    assert_eq!(cpu.ticks, 8);

    cpu.cpu_registers.pc = 0x0002;
    cpu.cpu_registers.f.register = 0x00; // Z = 0
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0); // the value should be the address of the next instruction
                                         // added to the offset
                                         // (old_pc + 2) + (-4) = (0x0002 + 2) - 4 = 0x00

    assert_eq!(cpu.ticks, 20);
}

#[test]
fn test_jr_nc() {
    let bus = TestBus::create(vec![
        0xFD, 0xDD, // Should jump here if NZ
        0x30, 0xFC, // Start here JR NZ 0xFR (which is -4)
        0xED, // move here if NZ condition is not satisfied
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0002;
    cpu.cpu_registers.f.register = 0x10; // C = 1
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.ticks, 8);

    cpu.cpu_registers.pc = 0x0002;
    cpu.cpu_registers.f.register = 0x00; // C = 0
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0);
    assert_eq!(cpu.ticks, 20);
}

#[test]
fn test_jr_z() {
    let bus = TestBus::create(vec![
        0xFD, 0xDD, // Should jump here if NZ
        0x28, 0xFC, // Start here JR NZ 0xFR (which is -4)
        0xED, // move here if NZ condition is not satisfied
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0002;
    cpu.cpu_registers.f.register = 0x00; // Z = 0
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.ticks, 8);

    cpu.cpu_registers.pc = 0x0002;
    cpu.cpu_registers.f.register = 0x80; // Z = 1
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0);
    assert_eq!(cpu.ticks, 20);
}

#[test]
fn test_jr_c() {
    let bus = TestBus::create(vec![
        0xFD, 0xDD, // Should jump here if NZ
        0x38, 0xFC, // Start here JR NZ 0xFR (which is -4)
        0xED, // move here if NZ condition is not satisfied
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0002;
    cpu.cpu_registers.f.register = 0x00; // C = 0
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.ticks, 8);

    cpu.cpu_registers.pc = 0x0002;
    cpu.cpu_registers.f.register = 0x10; // C = 1
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 0);
    assert_eq!(cpu.ticks, 20);
}
