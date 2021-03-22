use rand::Rng;

const START_ADDRESS: u16 = 0x200;
const VF: usize = 0x0F;

pub struct Cpu {
    pub reg: [u8; 16],
    i: u16,
    pub pc: u16,
    stack: Vec<u16>,
    pub dt: u8,
    pub st: u8,
    pub instr_time: u128,
    pub instr_timer: std::time::Instant,
    pub wait_for_input: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: [0; 16],
            i: 0,
            pc: START_ADDRESS,
            stack: Vec::<u16>::new(),
            dt: 0,
            st: 0,
            instr_time: 0,
            instr_timer: std::time::Instant::now(),
            wait_for_input: false,
        }
    }

    pub fn exec_instruction(&mut self, bus: &mut crate::bus::DataBus, instruction: u16) {
        let opcode: u8 = ((instruction & 0xF000) >> 12) as u8;
        let nnn: u16 = instruction & 0x0FFF;
        let kk: u8 = (instruction & 0x00FF) as u8;
        let n: u8 = (instruction & 0x000F) as u8;
        let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
        let y: u8 = ((instruction & 0x00F0) >> 4) as u8;

        match opcode {
            0x0 => match kk {
                0xE0 => self.cls(bus),
                0xEE => self.ret_s(),
                _ => panic!("{:#X} Not implemented", instruction),
            },
            0x1 => self.jp_addr(nnn),
            0x2 => self.call_addr(nnn),
            0x3 => self.se_vx_byte(x, kk),
            0x4 => self.sne_vx_byte(x, kk),
            0x5 => self.se_vx_vy(x, y),
            0x6 => self.ld_vx_byte(x, kk),
            0x7 => self.add_vx_byte(x, kk),
            0x8 => match n {
                0x0 => self.ld_vx_vy(x, y),
                0x1 => self.or_vx_vy(x, y),
                0x2 => self.and_vx_vy(x, y),
                0x3 => self.xor_vx_vy(x, y),
                0x4 => self.add_vx_vy(x, y),
                0x5 => self.sub_vx_vy(x, y),
                0x6 => self.shr_vx(x),
                0x7 => self.subn_vx_vy(x, y),
                0xE => self.shl_vx(x),
                _ => panic!("{:#X} Not implemented", instruction),
            },
            0x9 => self.sne_vx_vy(x, y),
            0xA => self.ld_i_addr(nnn),
            0xB => self.jmp_v0_addr(nnn),
            0xC => self.rnd_vx_byte(x, kk),
            0xD => self.d_xn_n(bus, x, y, n),
            0xE => match kk {
                0x9E => self.skp_vx(bus, x),
                0xA1 => self.sknp_vx(bus, x),
                _ => panic!("{:#X} Not implemented", instruction),
            },
            0xF => match kk {
                0x07 => self.ld_vx_dt(x),
                0x0A => self.ld_vx_k(bus, x),
                0x15 => self.ld_dt_vx(x),
                0x18 => self.ld_st_vx(x),
                0x29 => self.ld_f_vx(x),
                0x1E => self.add_i_vx(x),
                0x33 => self.ld_b_vx(bus, x),
                0x55 => self.ld_i_vx(bus, x),
                0x65 => self.ld_vx_i(bus, x),
                _ => panic!("{:#X} Not implemented", instruction),
            },
            _ => {
                panic!("{:#X} Not implemented", instruction);
            }
        };

        self.pc = self.pc + 2;
    }

    #[inline(always)]
    fn instr_exec_time(&mut self, delay: u16) {
        self.instr_time = delay as u128;
    }

    // 0x00E0 Clear screen
    fn cls(&mut self, bus: &mut crate::bus::DataBus) {
        bus.display.clear_screen();
        self.instr_exec_time(109);
    }

    // 0x00EE Return from a subroutine
    fn ret_s(&mut self) {
        let addr: u16 = self.stack.pop().unwrap();
        self.pc = addr - 2;
        self.instr_exec_time(109);
    }

    // 0x1 Jump to location nnn
    fn jp_addr(&mut self, nnn: u16) {
        self.pc = nnn - 2;
        self.instr_exec_time(105);
    }

    // 0x2 Call subroutine at nnn
    fn call_addr(&mut self, nnn: u16) {
        self.stack.push(self.pc + 2);
        self.pc = nnn - 2;
        self.instr_exec_time(105);
    }

    // 0x3 Skip next instruction if Vx = kk
    fn se_vx_byte(&mut self, x: u8, kk: u8) {
        if self.reg[x as usize] == kk {
            self.pc += 2;
            self.instr_exec_time(46);
            return;
        }
        self.instr_exec_time(64);
    }

    // 0x4 Skip next instruction if Vx != kk
    fn sne_vx_byte(&mut self, x: u8, kk: u8) {
        if self.reg[x as usize] != kk {
            self.pc += 2;
            self.instr_exec_time(46);
            return;
        }
        self.instr_exec_time(64);
    }

    // 0x5 Skip next instruction if Vx = Vy
    fn se_vx_vy(&mut self, x: u8, y: u8) {
        if self.reg[x as usize] == self.reg[y as usize] {
            self.pc += 2;
            self.instr_exec_time(64);
            return;
        }
        self.instr_exec_time(82);
    }

    // 0x6 Set Vx = kk
    fn ld_vx_byte(&mut self, vx: u8, byte: u8) {
        self.reg[vx as usize] = byte;
        self.instr_exec_time(27);
    }

    // 0x7 Set Vx = Vx + kk (wraps with no carry flag)
    fn add_vx_byte(&mut self, x: u8, kk: u8) {
        self.reg[x as usize] = self.reg[x as usize].wrapping_add(kk);
        self.instr_exec_time(45);
    }

    // 0x8__0 Set Vx = Vy
    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        self.reg[x as usize] = self.reg[y as usize];
        self.instr_exec_time(200);
    }

    // 0x8__1 Set Vx = Vx OR Vy
    fn or_vx_vy(&mut self, x: u8, y: u8) {
        self.reg[x as usize] |= self.reg[y as usize];
        self.instr_exec_time(200);
    }

    // 0x8__2 Set Vx = Vx AND Vy
    fn and_vx_vy(&mut self, x: u8, y: u8) {
        self.reg[x as usize] &= self.reg[y as usize];
        self.instr_exec_time(200);
    }

    // 0x8__3 Set Vx = Vx XOR Vy
    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        self.reg[x as usize] ^= self.reg[y as usize];
        self.instr_exec_time(200);
    }

    // 0x8__4 Set Vx = Vx + Vy, set VF = carry
    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let (res, carry) = self.reg[x as usize].overflowing_add(self.reg[y as usize]);
        self.reg[x as usize] = res;
        self.reg[VF] = carry as u8;
        self.instr_exec_time(200);
    }

    // 0x8__5 Set Vx = Vx - Vy (wrapping), Vf = 1 if Vx > Vy
    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        self.reg[VF] = if self.reg[x as usize] > self.reg[y as usize] {
            0x1
        } else {
            0x0
        };
        self.reg[x as usize] = self.reg[x as usize].wrapping_sub(self.reg[y as usize]);
        self.instr_exec_time(200);
    }

    // 0x8__6 Set Vx = Vx SHR 1
    // WARN: conflicting documentation. Using new implementation.
    fn shr_vx(&mut self, x: u8) {
        self.reg[VF] = self.reg[x as usize] & 0x01;
        self.reg[x as usize] = self.reg[x as usize].wrapping_shr(1);
        self.instr_exec_time(200);
    }

    // 0x8__7 Set Vx = Vy - Vx (wrapping), Vf = 1 if Vy > Vx
    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        self.reg[VF] = if self.reg[y as usize] > self.reg[x as usize] {
            0x1
        } else {
            0x0
        };
        self.reg[x as usize] = self.reg[y as usize].wrapping_sub(self.reg[x as usize]);
        self.instr_exec_time(200);
    }

    // 0x8__E Set Vx = Vx SHL 1
    // WARN: Conflicting documentation. Using new implementation.
    fn shl_vx(&mut self, x: u8) {
        self.reg[VF] = (self.reg[x as usize] & 0b10000000) >> 7;
        self.reg[x as usize] = self.reg[x as usize].wrapping_shl(1);
        self.instr_exec_time(200);
    }

    // 0x9XY0 Skip next instruction if Vx != Vy
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        if self.reg[x as usize] != self.reg[y as usize] {
            self.pc += 2;
            self.instr_exec_time(64);
            return;
        }
        self.instr_exec_time(82);
    }

    // 0xA Set I = nnn
    fn ld_i_addr(&mut self, nnn: u16) {
        self.i = nnn;
        self.instr_exec_time(55);
    }

    // 0xB Jump to location nnn + V0
    fn jmp_v0_addr(&mut self, nnn: u16) {
        self.pc = (nnn + self.reg[0] as u16) - 2;
        self.instr_exec_time(105);
    }

    // 0xC Set Vx = random byte AND kk
    fn rnd_vx_byte(&mut self, x: u8, kk: u8) {
        let mut rng = rand::thread_rng();
        self.reg[x as usize] = rng.gen_range(0..255) & kk;
        self.instr_exec_time(164);
    }

    // 0xD Dxyn - DRW Vx, Vy, nibble
    fn d_xn_n(&mut self, bus: &mut crate::bus::DataBus, x: u8, y: u8, n: u8) {
        let collision = bus.display.draw_s(
            self.reg[x as usize],
            self.reg[y as usize],
            &bus.ram.mem[self.i as usize..(self.i + n as u16) as usize],
        );
        self.reg[VF] = collision as u8; // VF set if collision found
        self.instr_exec_time(22743);
    }

    // E_9E Skip next instruction if key with the value of Vx is pressed
    fn skp_vx(&mut self, bus: &crate::bus::DataBus, x: u8) {
        if bus.keyboard.keys[self.reg[x as usize] as usize] {
            self.pc += 2;
            self.instr_exec_time(82);
            return;
        }
        self.instr_exec_time(64);
    }

    // E_A1 Skip next instruction if key with the value of Vx is not pressed
    fn sknp_vx(&mut self, bus: &crate::bus::DataBus, x: u8) {
        if !bus.keyboard.keys[self.reg[x as usize] as usize] {
            self.pc += 2;
            self.instr_exec_time(82);
            return;
        }
        self.instr_exec_time(64);
    }

    // 0xF_07 Set Vx = delay timer value
    fn ld_vx_dt(&mut self, x: u8) {
        self.reg[x as usize] = self.dt;
        self.instr_exec_time(45);
    }

    // 0xF_0A Wait for a key press (blocking), store the value of the key in Vx
    fn ld_vx_k(&mut self, bus: &mut crate::bus::DataBus, x: u8) {
        bus.keyboard.register = x;
        self.wait_for_input = true;
    }

    // 0xF_15 Set delay timer = Vx
    fn ld_dt_vx(&mut self, x: u8) {
        self.dt = self.reg[x as usize];
        self.instr_exec_time(45);
    }

    // 0xF_18 Set sound timer = Vx
    fn ld_st_vx(&mut self, x: u8) {
        self.st = self.reg[x as usize];
        self.instr_exec_time(45);
    }

    // 0xF_1E Set I = I + Vx
    fn add_i_vx(&mut self, x: u8) {
        self.i += self.reg[x as usize] as u16;
        self.instr_exec_time(86);
    }

    // 0xF_29 Set I = location of sprite for digit Vx
    fn ld_f_vx(&mut self, x: u8) {
        self.i = self.reg[x as usize] as u16 * 5;
        self.instr_exec_time(91);
    }

    // 0xF_33 Store BCD representation of Vx in memory locations I, I+1, and I+2
    fn ld_b_vx(&mut self, bus: &mut crate::bus::DataBus, x: u8) {
        let hundreds = self.reg[x as usize] / 100;
        let tens = (self.reg[x as usize] % 100) / 10;
        let units = self.reg[x as usize] % 10;
        bus.ram.write_byte(self.i as usize, hundreds);
        bus.ram.write_byte((self.i + 1) as usize, tens);
        bus.ram.write_byte((self.i + 2) as usize, units);
        self.instr_exec_time((hundreds + tens + units) as u16 * 73 + 364);
    }

    // 0xF_55 Store registers V0 through Vx in memory starting at location I
    //WARN: Conflicting documentation.
    fn ld_i_vx(&mut self, bus: &mut crate::bus::DataBus, x: u8) {
        let i = self.i as usize;
        let x = x as usize;
        bus.ram.mem[i..i + x + 1].copy_from_slice(&self.reg[0..x + 1]);
        //self.i += x as u16 + 1;
        self.instr_exec_time((x as u16 * 64) + 64);
    }

    // 0xF_65 Read registers V0 through Vx from memory starting at location I
    // WARN: Conflicting documentation.
    fn ld_vx_i(&mut self, bus: &crate::bus::DataBus, x: u8) {
        let i = self.i as usize;
        let x = x as usize;
        self.reg[0..x + 1].copy_from_slice(&bus.ram.mem[i..i + x + 1]);
        //self.i += x as u16 + 1;
        self.instr_exec_time((x as u16 * 64) + 64);
    }
}
