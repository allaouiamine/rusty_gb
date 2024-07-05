use std::{env, process::exit, sync::{Arc, Mutex}};

use rusty_gb::{bus::GbBus, cpu::CpuContext, emu::EmuContext};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("Usage: {} <rom_file>\n", args[0]);
        exit(1);
    }

    let bus = Arc::new(Mutex::new(GbBus::new(args[1].clone())));
    let mut cpu_context = CpuContext::new(bus);
    EmuContext.run(&mut cpu_context);
}
