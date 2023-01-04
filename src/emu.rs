use crate::cpu::CpuContext;

pub struct EmuContext {
    paused: bool,
    running: bool,
    ticks: usize,
}

impl EmuContext {
    pub fn new() -> Self {
        Self {
            paused: false,
            running: false,
            ticks: 0,
        }
    }
    pub fn get_ticks(&self) -> usize {
        self.ticks
    }
    pub fn delay(&self, _: usize) {
        unimplemented!()
    }

    pub fn run(&mut self, cpu: &mut CpuContext) {
        // init UI

        // starting emulation
        self.running = true;
        println!("Emulation started!");
        println!("Emulation started!");

        while self.running {
            if self.paused {
                // self.delay(10);
                unimplemented!();
            }

            if !cpu.cpu_step() {
                // println!("CPU Stopped!");
                // exit(3);
            }
            self.ticks += 1;
        }
    }

    pub fn emu_cycles(&mut self, cpu_cycles: usize) {
        self.ticks += cpu_cycles;
    }
}
