#[derive(Debug)]
pub struct DMA {
    pub active: bool,
    pub byte: u8,
    pub value: u8,
    pub start_delay: u8,
}

impl DMA {
    pub fn new() -> Self {
        Self {
            active: false,
            byte: 0,
            value: 0,
            start_delay: 0,
        }
    }

    pub fn dma_start(&mut self, start: u8) {
        self.active = true;
        self.byte = 0;
        self.start_delay = 2;
        self.value = start;
    }

    pub fn dma_is_transferring(&self) -> bool {
        self.active
    }
}
