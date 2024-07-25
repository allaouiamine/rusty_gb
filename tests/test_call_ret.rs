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

#[test]
fn test_ret_nz_ok() {
    let bus = TestBus::create(vec![
        0xC0, // RET NZ
        0xFD, //STACK
        0x22, 0x33,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x00; // Z=0
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x0002;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    assert_eq!(cpu.ticks, 20);
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
}

#[test]
fn test_ret_nz_nok() {
    let bus = TestBus::create(vec![
        0xC0, // RET NZ
        0xFD, //STACK
        0x22, 0x33,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x80; // Z=1
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x0002;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 8);
    assert_eq!(cpu.cpu_registers.pc, 0x1);
}

#[test]
fn test_ret_z_nok() {
    let bus = TestBus::create(vec![
        0xC8, // RET Z
        0xFD, //STACK
        0x22, 0x33,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x00; // Z=0
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x0002;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 20);
    assert_eq!(cpu.cpu_registers.pc, 0x1);
}

#[test]
fn test_ret_z_ok() {
    let bus = TestBus::create(vec![
        0xC8, // RET Z
        0xFD, //STACK
        0x22, 0x33,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x80; // Z=1
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x0002;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 8);
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
}

#[test]
fn test_call_nz_ok() {
    let bus = TestBus::create(vec![
        0xC4, 0x08, 0x00, // CALL NZ,0x0008
        0xFD, 0xFD, 0xFD, 0xFD, 0xFD, // INVALID
        0x00, // Beginning of the function
        0xC9, // RET
        0xFD, //STACK
        0x00, 0x00,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x00;
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x000D;
    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    // assert_eq!(cpu.ticks, 24);
    assert_eq!(cpu.cpu_registers.pc, 0x0008);
    assert_eq!(cpu.bus_read16(cpu.cpu_registers.sp), 0x0003); // Address of the instruction after
                                                              // CALL
    assert_eq!(cpu.cpu_registers.sp, 0x000B);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 4);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 16);
    assert_eq!(cpu.cpu_registers.pc, 0x0003);
    assert_eq!(cpu.cpu_registers.sp, 0x000D);
}

#[test]
fn test_call_nz_nok() {
    let bus = TestBus::create(vec![
        0xC4, 0x08, 0x00, // CALL NZ,0x0008
        0xFD, 0xFD, 0xFD, 0xFD, 0xFD, // INVALID
        0x00, // Beginning of the function
        0xC9, // RET
        0xFD, //STACK
        0x00, 0x00,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x80; // Z = 1
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x000D;
    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 16);
    assert_eq!(cpu.cpu_registers.pc, 0x0003);
    assert_eq!(cpu.cpu_registers.sp, 0x000D);
}


#[test]
fn test_ret_nc_ok() {
    let bus = TestBus::create(vec![
        0xD0, // RET NC
        0xFD, //STACK
        0x22, 0x33,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x00; // C=0
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x0002;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    assert_eq!(cpu.ticks, 20);
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
}

#[test]
fn test_ret_nc_nok() {
    let bus = TestBus::create(vec![
        0xD0, // RET NC
        0xFD, //STACK
        0x22, 0x33,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x10; // C=1
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x0002;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 8);
    assert_eq!(cpu.cpu_registers.pc, 0x1);
}

#[test]
fn test_ret_c_nok() {
    let bus = TestBus::create(vec![
        0xD8, // RET C
        0xFD, //STACK
        0x22, 0x33,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x00; // C=0
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x0002;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 20);
    assert_eq!(cpu.cpu_registers.pc, 0x1);
}

#[test]
fn test_ret_c_ok() {
    let bus = TestBus::create(vec![
        0xD8, // RET Z
        0xFD, //STACK
        0x22, 0x33,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x10; // C=1
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x0002;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 8);
    assert_eq!(cpu.cpu_registers.pc, 0x3322);
}

#[test]
fn test_call_z_ok() {
    let bus = TestBus::create(vec![
        0xCC, 0x08, 0x00, // CALL Z,0x0008
        0xFD, 0xFD, 0xFD, 0xFD, 0xFD, // INVALID
        0x00, // Beginning of the function
        0xC9, // RET
        0xFD, //STACK
        0x00, 0x00,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x80;
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x000D;
    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    // assert_eq!(cpu.ticks, 24);
    assert_eq!(cpu.cpu_registers.pc, 0x0008);
    assert_eq!(cpu.bus_read16(cpu.cpu_registers.sp), 0x0003); // Address of the instruction after
                                                              // CALL
    assert_eq!(cpu.cpu_registers.sp, 0x000B);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 4);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 16);
    assert_eq!(cpu.cpu_registers.pc, 0x0003);
    assert_eq!(cpu.cpu_registers.sp, 0x000D);
}

#[test]
fn test_call_z_nok() {
    let bus = TestBus::create(vec![
        0xCC, 0x08, 0x00, // CALL Z,0x0008
        0xFD, 0xFD, 0xFD, 0xFD, 0xFD, // INVALID
        0x00, // Beginning of the function
        0xC9, // RET
        0xFD, //STACK
        0x00, 0x00,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x00;
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x000D;
    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 16);
    assert_eq!(cpu.cpu_registers.pc, 0x0003);
    assert_eq!(cpu.cpu_registers.sp, 0x000D);
}

#[test]
fn test_call_nc_ok() {
    let bus = TestBus::create(vec![
        0xD4, 0x08, 0x00, // CALL NC,0x0008
        0xFD, 0xFD, 0xFD, 0xFD, 0xFD, // INVALID
        0x00, // Beginning of the function
        0xC9, // RET
        0xFD, //STACK
        0x00, 0x00,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x00;
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x000D;
    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    // assert_eq!(cpu.ticks, 24);
    assert_eq!(cpu.cpu_registers.pc, 0x0008);
    assert_eq!(cpu.bus_read16(cpu.cpu_registers.sp), 0x0003); // Address of the instruction after
                                                              // CALL
    assert_eq!(cpu.cpu_registers.sp, 0x000B);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 4);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 16);
    assert_eq!(cpu.cpu_registers.pc, 0x0003);
    assert_eq!(cpu.cpu_registers.sp, 0x000D);
}

#[test]
fn test_call_nc_nok() {
    let bus = TestBus::create(vec![
        0xD4, 0x08, 0x00, // CALL NC,0x0008
        0xFD, 0xFD, 0xFD, 0xFD, 0xFD, // INVALID
        0x00, // Beginning of the function
        0xC9, // RET
        0xFD, //STACK
        0x00, 0x00,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x10;
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x000D;
    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 16);
    assert_eq!(cpu.cpu_registers.pc, 0x0003);
    assert_eq!(cpu.cpu_registers.sp, 0x000D);
}

#[test]
fn test_call_c_ok() {
    let bus = TestBus::create(vec![
        0xDC, 0x08, 0x00, // CALL C,0x0008
        0xFD, 0xFD, 0xFD, 0xFD, 0xFD, // INVALID
        0x00, // Beginning of the function
        0xC9, // RET
        0xFD, //STACK
        0x00, 0x00,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x10;
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x000D;
    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    // assert_eq!(cpu.ticks, 24);
    assert_eq!(cpu.cpu_registers.pc, 0x0008);
    assert_eq!(cpu.bus_read16(cpu.cpu_registers.sp), 0x0003); // Address of the instruction after
                                                              // CALL
    assert_eq!(cpu.cpu_registers.sp, 0x000B);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.ticks, 4);
    cpu.ticks = 0;

    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 16);
    assert_eq!(cpu.cpu_registers.pc, 0x0003);
    assert_eq!(cpu.cpu_registers.sp, 0x000D);
}

#[test]
fn test_call_c_nok() {
    let bus = TestBus::create(vec![
        0xDC, 0x08, 0x00, // CALL NZ,0x0008
        0xFD, 0xFD, 0xFD, 0xFD, 0xFD, // INVALID
        0x00, // Beginning of the function
        0xC9, // RET
        0xFD, //STACK
        0x00, 0x00,
    ]);
    let mut cpu = CpuContext::new(bus);
    cpu.cpu_registers.f.register = 0x00;
    cpu.cpu_registers.pc = 0x0000;
    cpu.cpu_registers.sp = 0x000D;
    let _ = cpu.cpu_step().unwrap();
    // TODO: Check the number of ticks
    //assert_eq!(cpu.ticks, 16);
    assert_eq!(cpu.cpu_registers.pc, 0x0003);
    assert_eq!(cpu.cpu_registers.sp, 0x000D);
}
