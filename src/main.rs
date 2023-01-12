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

    let instruction_set = InstructionSet::new();
    let mut cpu_context = CpuContext::new(&args[1][..], &instruction_set);
    EmuContext.run(&mut cpu_context);
}
