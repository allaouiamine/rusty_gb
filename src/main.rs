use std::{env, process::exit};

use rusty_gb::{
    cpu::{instruction_set::InstructionSet, CpuContext},
    emu::EmuContext,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("Usage: {} <rom_file>\n", args[0]);
        exit(1);
    }

    let mut emu_context = EmuContext::new();

    let instruction_set = InstructionSet::new();
    let mut cpu_context = CpuContext::new(&args[1][..], &instruction_set);
    // let mut cpu_context = CpuContext::new("../gb_emulator/roms/01-special.gb", &instruction_set);
    emu_context.run(&mut cpu_context);
}

/*
 * 560489c560489
< 001A8B34 - 0050: INC A        (3C C9 00) A: 04 F: ---C BC: 0100 DE: C7BA HL: 9000
---
> 001A8B3C - 0050: INC A        (3C C9 00) A: 04 F: ---C BC: 0100 DE: C7BA HL: 9000
*/
