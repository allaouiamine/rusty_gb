use crate::cpu::CpuContext;

pub struct EmuContext;

impl EmuContext {
    pub fn run(&mut self, cpu: &mut CpuContext) -> anyhow::Result<()> {
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
