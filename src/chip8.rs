use super::bus;
use super::cpu;
use std::time::Instant;

pub struct Chip8 {
    pub cpu: cpu::Cpu,
    pub bus: bus::DataBus,
    pub delay_timer: std::time::Instant,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            bus: bus::DataBus::new(),
            delay_timer: std::time::Instant::now(),
        }
    }

    pub fn exec_cycle(&mut self) {
        if self.cpu.instr_timer.elapsed().as_micros() > self.cpu.instr_time {
            let instruction = self.fetch_instruction();
            self.cpu.exec_instruction(&mut self.bus, instruction);
            self.cpu.instr_timer = Instant::now();
        }
        self.dec_dt_st();
    }

    fn dec_dt_st(&mut self) {
        // DEC dt and st at 60hz
        if self.delay_timer.elapsed().as_nanos() > 16666667 {
            if self.cpu.dt > 0 {
                self.cpu.dt -= 1;
            }
            if self.cpu.st > 0 {
                self.cpu.st -= 1;
            }
            self.delay_timer = Instant::now();
        }
    }

    fn fetch_instruction(&self) -> u16 {
        let byte1: u8 = self.bus.ram.read_byte(self.cpu.pc as usize);
        let byte2: u8 = self.bus.ram.read_byte(self.cpu.pc as usize + 1);

        (byte1 as u16) << 8 | byte2 as u16
    }

    pub fn load_mem(&mut self, rom: &Vec<u8>, offset: usize) {
        for (i, value) in rom.iter().enumerate() {
            self.bus.ram.write_byte(offset + i, *value)
        }
    }
}
