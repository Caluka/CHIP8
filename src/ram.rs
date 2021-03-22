pub struct Ram {
    pub mem: [u8; 4096],
}

impl Ram {
    pub fn new() -> Self {
        Self { mem: [0; 4096] }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.mem[address] = value;
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.mem[address]
    }
}
