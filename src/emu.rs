use std::{thread, time::Duration};

use crate::{cpu::CpuContext, ui::UI};

use minifb::Key;

pub struct EmuContext;

impl EmuContext {
    pub fn delay(&self, _: usize) {
        unimplemented!()
    }

    pub fn run(&mut self, cpu: &mut CpuContext) {
        // while cpu.ticks < 0x00031688 {
        // cpu.cpu_step();
        // }

        let mut ui = UI::new(16 * 8, 24 * 8, minifb::Scale::X8);
        while ui.dbg_window.is_open() && !ui.dbg_window.is_key_down(Key::Escape) {
            cpu.cpu_step();

            if cpu.dma_done {
                ui.update(cpu, true);
                thread::sleep(Duration::from_secs(10));
            } else {
                if let Some(address) = cpu.last_written_address {
                    if address > 0x8000 && address < 0x9000 {
                        ui.update(&cpu, false);
                        thread::sleep(Duration::from_nanos(2));
                    }
                }
            }
        }
    }
}
