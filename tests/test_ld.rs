mod common;

use common::util::TestBus;
use rusty_gb::cpu::{
    registers::{CpuRegisters, FlagsRegister},
    CpuContext, RegisterType,
};

#[test]
fn test_ld_0x0() {
    let mut bus_data = vec![0xFD; 0x10000];
    let rom = vec![
        0x01, 0x11, 0x12, // LD BC, 0x1211
        0x02, // <- PC=3 ; LD (BC), A
        0x06, 0x13, // <- PC=4; LD B, 0x13
        0x08, 0x14, 0x15, // <- PC=6;  LD (0x1514), SP
        0x0A, // <- PC=9; LD A, (BC)
        0x0E, 0x16, // LD C, 0x16
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers = CpuRegisters {
        a: 0x99,
        f: FlagsRegister { register: 0 },
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        pc: 0,
        sp: 0xFFFF,
    };

    // LD BC, 0x1211
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::BC), 0x1211);
    assert_eq!(cpu.cpu_registers.pc, 3);
    // TODO: Fix later it should be 12 not 16
    //assert_eq!(cpu.ticks, 12);

    // LD (BC), A
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.bus.lock().unwrap().bus_read(0x1211), 0x99); // BC = 0x1211, (BC) = 0x99
    assert_eq!(cpu.ticks, 8);

    // LD B, 0x13
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.cpu_registers.b, 0x13);
    assert_eq!(cpu.ticks, 8);

    // LD (0x1514), SP
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 9);
    let lo = cpu.bus.lock().unwrap().bus_read(0x1514) as u16;
    let hi = cpu.bus.lock().unwrap().bus_read(0x1515) as u16;
    assert_eq!(lo | (hi << 8), 0xFFFF); // LD (0x1514), SP
    assert_eq!(cpu.ticks, 20);

    // LD A, (BC)
    cpu.ticks = 0;
    cpu.bus.lock().unwrap().bus_write(0x1311, 0xAA);
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 10);
    assert_eq!(cpu.cpu_registers.a, 0xAA); // BC = 0x1311
    assert_eq!(cpu.ticks, 8);

    // LD C, 0x16
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 12);
    assert_eq!(cpu.cpu_registers.c, 0x16);
    assert_eq!(cpu.ticks, 8);
}

#[test]
fn test_ld_0x1() {
    let mut bus_data = vec![0xFD; 0x10000];
    let rom = vec![
        0x11, 0x17, 0x18, // LD DE, 0x1817
        0x12, // LD (DE), A
        0x16, 0x19, // LD D, 0x19
        0x1A, // LD A, (DE)
        0x1E, 0x20, // LD E, 0x20
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers = CpuRegisters {
        a: 0x99,
        f: FlagsRegister { register: 0 },
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        pc: 0,
        sp: 0xFFFF,
    };

    // LD DE, 0x1817
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 3);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::DE), 0x1817);
    // TODO: Fix later it should be 12 not 16
    //assert_eq!(cpu.ticks, 12);

    // LD (DE), A
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.bus.lock().unwrap().bus_read(0x1817), 0x99);
    assert_eq!(cpu.ticks, 8);

    // LD D, 0x19
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.cpu_registers.d, 0x19);
    assert_eq!(cpu.ticks, 8);

    // LD A, (DE)
    cpu.ticks = 0;
    cpu.bus.lock().unwrap().bus_write(0x1917, 0xBB);
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 7);
    assert_eq!(cpu.cpu_registers.a, 0xBB);

    // LD E, 0x20
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 9);
    assert_eq!(cpu.cpu_registers.e, 0x20);
    assert_eq!(cpu.ticks, 8);
}

#[test]
fn test_ld_0x2() {
    let mut bus_data = vec![0xFD; 0x10000];
    let rom = vec![
        0x21, 0x21, 0x22, // LD HL, 0x2211
        0x22, // LD (HL+), A
        0x26, 0x23, // LD H, 0x23
        0x2A, // LD A, (HL+)
        0x2E, 0x24, // LD L, 0x24
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers = CpuRegisters {
        a: 0x99,
        f: FlagsRegister { register: 0 },
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        pc: 0,
        sp: 0xFFFF,
    };

    // LD HL, 0x2211
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 3);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x2221);
    // TODO: Fix later it should be 12 not 16
    //assert_eq!(cpu.ticks, 12);

    // LD (HL+), A
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.bus.lock().unwrap().bus_read(0x2221), 0x99);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x2222);
    assert_eq!(cpu.ticks, 8);

    // LD H, 0x23
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.cpu_registers.h, 0x23);
    assert_eq!(cpu.ticks, 8);

    // LD A, (HL+)
    cpu.ticks = 0;
    cpu.bus.lock().unwrap().bus_write(0x2322, 0xBB);
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 7);
    assert_eq!(cpu.cpu_registers.a, 0xBB);
    assert_eq!(cpu.cpu_registers.l, 0x23);

    // LD L, 0x24
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 9);
    assert_eq!(cpu.cpu_registers.l, 0x24);
    assert_eq!(cpu.ticks, 8);
}

#[test]
fn test_ld_0x3() {
    let mut bus_data = vec![0xfd; 0x10000];
    let rom = vec![
        0x31, 0x25, 0x26, // ld sp, 0x2625
        0x32, // ld (hl-), a
        0x36, 0x27, // ld (hl), 0x27
        0x3a, // ld a, (hl-)
        0x3e, 0x28, // ld a, 0x28
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers = CpuRegisters {
        a: 0x99,
        f: FlagsRegister { register: 0 },
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0x22,
        l: 0x21,
        pc: 0,
        sp: 0xffff,
    };

    // LD SP, 0x2625
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 3);
    assert_eq!(cpu.cpu_registers.sp, 0x2625);
    // TODO: Fix later it should be 12 not 16
    //assert_eq!(cpu.ticks, 12);

    // LD (HL-), A
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.bus.lock().unwrap().bus_read(0x2221), 0x99);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x2220);
    assert_eq!(cpu.ticks, 8);

    // LD (HL), 0x27
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.bus.lock().unwrap().bus_read(0x2220), 0x27);
    assert_eq!(cpu.ticks, 12);

    // LD A, (HL-)
    cpu.ticks = 0;
    cpu.bus.lock().unwrap().bus_write(0x2220, 0xBB);
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 7);
    assert_eq!(cpu.cpu_registers.a, 0xBB);
    assert_eq!(cpu.cpu_registers.l, 0x1F);
    assert_eq!(cpu.cpu_registers.get_register_16(&RegisterType::HL), 0x221F);
    assert_eq!(cpu.ticks, 8);

    // LD A, 0x28
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 9);
    assert_eq!(cpu.cpu_registers.a, 0x28);
    assert_eq!(cpu.ticks, 8);
}

#[test]
fn test_ld_to_b() {
    let mut bus_data = vec![0xfd; 0x10000];
    let rom = vec![
        0x40, // LD B, B
        0x41, // LD B, C
        0x42, // LD B, D
        0x43, // LD B, E
        0x44, // LD B, H
        0x45, // LD B, L
        0x46, // LD B, (HL)
        0x47, // LD B, A
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers = CpuRegisters {
        a: 0x11,
        f: FlagsRegister { register: 0 },
        b: 0x22,
        c: 0x33,
        d: 0x44,
        e: 0x55,
        h: 0x66,
        l: 0x77,
        pc: 0,
        sp: 0xffff,
    };

    // LD B, B
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 1);
    assert_eq!(cpu.cpu_registers.b, 0x22);
    assert_eq!(cpu.ticks, 4);

    // LD B, C
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 2);
    assert_eq!(cpu.cpu_registers.b, 0x33);
    assert_eq!(cpu.ticks, 4);

    // LD B, D
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 3);
    assert_eq!(cpu.cpu_registers.b, 0x44);
    assert_eq!(cpu.ticks, 4);

    // LD B, E
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.cpu_registers.b, 0x55);
    assert_eq!(cpu.ticks, 4);

    // LD B, H
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 5);
    assert_eq!(cpu.cpu_registers.b, 0x66);
    assert_eq!(cpu.ticks, 4);

    // LD B, L
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.cpu_registers.b, 0x77);
    assert_eq!(cpu.ticks, 4);

    // LD B, (HL)
    cpu.ticks = 0;
    cpu.bus.lock().unwrap().bus_write(0x6677, 0xBB);
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 7);
    assert_eq!(cpu.cpu_registers.b, 0xBB);
    assert_eq!(cpu.ticks, 8);

    // LD B, A
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 8);
    assert_eq!(cpu.cpu_registers.b, 0x11);
    assert_eq!(cpu.ticks, 4);
}

#[test]
fn test_ld_to_c() {
    let mut bus_data = vec![0xfd; 0x10000];
    let rom = vec![
        0x48, // LD C, B
        0x49, // LD C, C
        0x4A, // LD C, D
        0x4B, // LD C, E
        0x4C, // LD C, H
        0x4D, // LD C, L
        0x4E, // LD C, (HL)
        0x4F, // LD C, A
    ];
    let l = rom.len();
    bus_data[..l].clone_from_slice(&rom);
    let bus = TestBus::create(bus_data);
    let mut cpu = CpuContext::new(bus);
    cpu.old_pc = 0;

    cpu.cpu_registers = CpuRegisters {
        a: 0x11,
        f: FlagsRegister { register: 0 },
        b: 0x22,
        c: 0x33,
        d: 0x44,
        e: 0x55,
        h: 0x66,
        l: 0x77,
        pc: 0,
        sp: 0xffff,
    };

    // LD C, B
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 1);
    assert_eq!(cpu.cpu_registers.c, 0x22);
    assert_eq!(cpu.ticks, 4);

    // LD C, C
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 2);
    assert_eq!(cpu.cpu_registers.c, 0x22);
    assert_eq!(cpu.ticks, 4);

    // LD C, D
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 3);
    assert_eq!(cpu.cpu_registers.c, 0x44);
    assert_eq!(cpu.ticks, 4);

    // LD C, E
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 4);
    assert_eq!(cpu.cpu_registers.c, 0x55);
    assert_eq!(cpu.ticks, 4);

    // LD C, H
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 5);
    assert_eq!(cpu.cpu_registers.c, 0x66);
    assert_eq!(cpu.ticks, 4);

    // LD C, L
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 6);
    assert_eq!(cpu.cpu_registers.c, 0x77);
    assert_eq!(cpu.ticks, 4);

    // LD C, (HL)
    cpu.ticks = 0;
    cpu.bus.lock().unwrap().bus_write(0x6677, 0xCC);
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 7);
    assert_eq!(cpu.cpu_registers.c, 0xCC);
    assert_eq!(cpu.ticks, 8);

    // LD C, A
    cpu.ticks = 0;
    let _ = cpu.cpu_step().unwrap();
    assert_eq!(cpu.cpu_registers.pc, 8);
    assert_eq!(cpu.cpu_registers.c, 0x11);
    assert_eq!(cpu.ticks, 4);
}
