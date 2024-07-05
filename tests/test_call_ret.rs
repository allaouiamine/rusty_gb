mod common;

use common::util::TestBus;
use rusty_gb::cpu::CpuContext;

#[test]
fn test_call() {
    let bus = TestBus::create(vec![
        0xCD, 0x08, 0x00, // CALL 0x0008
        0xFD, 0xFD, 0xFD, 0xFD, 0xFD, // INVALID
        0x00, // Beginning of the function
        0xC9, // RET
        0xFD, //STACK
        0x00, 0x00,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x000D;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 24);
    assert_eq!(cpu.cpu_registers.pc, 0x0008);
    assert_eq!(cpu.bus_read16(cpu.cpu_registers.sp), 0x0003); // Address of the instruction after
                                                              // CALL
    assert_eq!(cpu.cpu_registers.sp, 0x000B);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 4);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 16);
    assert_eq!(cpu.cpu_registers.pc, 0x0003);
    assert_eq!(cpu.cpu_registers.sp, 0x000D);
}
