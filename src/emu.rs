use std::sync::Arc;
use std::process;
use std::thread;

use crate::cpu::CpuContext;
use crate::ui::UI;

pub struct EmuContext;

impl EmuContext {
    pub fn run(&mut self, cpu: &mut CpuContext) -> anyhow::Result<()> {
        let bus_clone = Arc::clone(&cpu.bus);
        thread::spawn(move || {
            let mut ui = UI::new(16 * 8, 24 * 8, minifb::Scale::X4, bus_clone).unwrap();
            match ui.run() {
                Ok(_) => process::exit(0),
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    process::exit(1);
                }
            }
        });

        loop {
            match cpu.cpu_step() {
                Ok(_) => {}
                Err(err) => {
                    dbg!(&cpu.current_instruction);
                    anyhow::bail!(err);
                }
            }
        }
    }
}
