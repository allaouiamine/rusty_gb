use core::time;
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crate::{
    cpu::{util::ValueEnum, CpuContext},
    ui::UI,
};

use minifb::{Key, Scale};

pub struct EmuContext;

pub struct SharedData {
    pub tile_number: usize,
    pub tile: [u8; 16],
}

impl EmuContext {
    pub fn run(&mut self, cpu: &mut CpuContext) {
        let mut ui = UI::new(16 * 8, 24 * 8, Scale::X4);

        let mut stat_cpu = Stats::new();
        let mut stat_ui = Stats::new();
        let mut now;
        let mut elapsed;
        while ui.dbg_window.is_open() && !ui.dbg_window.is_key_down(Key::Escape) {
            now = Instant::now();
            cpu.cpu_step();
            elapsed = now.elapsed().subsec_nanos();
            stat_cpu.put(elapsed);

            if stat_cpu.count > 100_000 {
                dbg!(stat_cpu);
                dbg!(stat_ui);
                return;
            }
            now = Instant::now();
            if let Some(address) = cpu.last_written_address {
                if address > 0x8000 && address < 0xA000 {
                    ui.update(cpu);
                }
            }
            elapsed = now.elapsed().subsec_nanos();
            stat_ui.put(elapsed);
        }
    }
}

#[derive(Debug)]
pub struct Stats {
    min: u32,
    max: u32,
    average: u32,
    count: u32,
    sum: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            min: 1_000_000_000,
            max: 0,
            average: 0,
            count: 0,
            sum: 0,
        }
    }
    pub fn put(&mut self, elapsed: u32) {
        self.count += 1;
        self.min = if self.min < elapsed {
            self.min
        } else {
            elapsed
        };
        self.max = if self.max < elapsed {
            elapsed
        } else {
            self.max
        };
        self.sum += elapsed;
        self.average = self.sum / self.count;
    }
}
