use std::env;
use std::fs::File;
use std::io::Read;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use chip8::Chip8;

mod bus;
mod chip8;
mod cpu;
mod display;
mod kb;
mod ram;

fn main() {
    let filename = env::args().nth(1).unwrap_or("games/INVADERS".to_string());
    let mut font_data = Vec::<u8>::new();
    let mut game_data = Vec::<u8>::new();

    let mut font_file = File::open("FONTS.chip8").unwrap();
    let mut game_file = File::open(filename).unwrap();

    font_file.read_to_end(&mut font_data).unwrap();
    game_file.read_to_end(&mut game_data).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_mem(&font_data, 0x0);
    chip8.load_mem(&game_data, 0x200);

    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Chip-8 Emulator")
        .with_inner_size(LogicalSize::new(640, 320))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let mut pixels = Pixels::new(640, 320, surface_texture).unwrap();

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            chip8.bus.keyboard.handle_keyboard(&mut input, control_flow);
        }

        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame(), &chip8.bus.display.framebuffer);
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Block cycle execution if waiting for input
        if chip8.cpu.wait_for_input {
            for (i, key) in chip8.bus.keyboard.keys.iter().enumerate() {
                if *key {
                    chip8.cpu.reg[chip8.bus.keyboard.register as usize] = i as u8;
                    chip8.cpu.wait_for_input = false;
                    chip8.cpu.instr_timer = std::time::Instant::now();
                }
            }
        } else {
            chip8.exec_cycle();
        }

        if chip8.bus.display.req_draw {
            window.request_redraw();
            chip8.bus.display.req_draw = false;
        }
    });
}

fn draw(frame: &mut [u8], buffer: &[[bool; 64]; 32]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % 640 as usize;
        let y = i / 640 as usize;

        let rgba = if buffer[y / 10][x / 10] == true {
            [0xFF, 0xFF, 0xFF, 0xFF]
        } else {
            [0x0, 0x0, 0x0, 0xFF]
        };

        pixel.copy_from_slice(&rgba);
    }
}
