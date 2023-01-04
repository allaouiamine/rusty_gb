pub struct Serial {
    serial_transfer_data: u8,    // 0xFF01
    serial_transfer_control: u8, //0xFF02
}

impl Serial {
    pub fn new() -> Self {
        Self {
            serial_transfer_control: 0,
            serial_transfer_data: 0,
        }
    }
    pub fn io_read(&self, address: u16) -> u8 {
        if address == 0xFF01 {
            self.serial_transfer_data
        } else if address == 0xFF02 {
            self.serial_transfer_control
        } else {
            unimplemented!();
        }
    }
    pub fn io_write(&mut self, address: u16, value: u8) {
        if address == 0xFF01 {
            self.serial_transfer_data = value;
        } else if address == 0xFF02 {
            self.serial_transfer_control = value;
        } else {
            unimplemented!();
        }
    }
}
