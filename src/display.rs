pub struct Display {
    pub framebuffer: [[bool; 64]; 32],
    pub req_draw: bool,
}

impl Display {
    pub fn new() -> Self {
        Self {
            framebuffer: [[false; 64]; 32],
            req_draw: false,
        }
    }

    pub fn draw_s(&mut self, vx: u8, vy: u8, bytes: &[u8]) -> bool {
        let mut collision = false;
        for j in 0..bytes.len() {
            let row = bytes[j];
            for i in 0..8 {
                let new_pixel = row >> (7 - i) & 0x01;
                if new_pixel == 1 {
                    let xi = (vx.wrapping_add(i)) % 64;
                    let yj = (vy.wrapping_add(j as u8)) % 32;
                    let old_pixel = self.get_pixel(xi as usize, yj as usize);
                    if old_pixel {
                        collision = true;
                    }
                    self.set_pixel(xi as usize, yj as usize, (new_pixel == 1) ^ old_pixel);
                }
            }
        }
        self.req_draw = true;
        return collision;
    }

    #[allow(dead_code)]
    pub fn debug_draw(&self) {
        for col in self.framebuffer.iter() {
            for c in col.iter() {
                if *c == true {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!("");
        }
    }

    pub fn clear_screen(&mut self) {
        self.framebuffer = [[false; 64]; 32];
        self.req_draw = true;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.framebuffer[y][x] = value;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.framebuffer[y][x] == true
    }
}
