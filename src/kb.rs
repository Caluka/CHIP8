use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;

pub struct Keyboard {
    pub keys: [bool; 16],
    pub register: u8,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
            register: 0,
        }
    }

    pub fn handle_keyboard(
        &mut self,
        input: &mut WinitInputHelper,
        control_flow: &mut ControlFlow,
    ) {
        if input.key_pressed(VirtualKeyCode::Key1) || input.key_held(VirtualKeyCode::Key1) {
            self.keys[0] = true;
        }
        if input.key_pressed(VirtualKeyCode::Key2) || input.key_held(VirtualKeyCode::Key2) {
            self.keys[1] = true;
        }
        if input.key_pressed(VirtualKeyCode::Key3) || input.key_held(VirtualKeyCode::Key3) {
            self.keys[2] = true;
        }
        if input.key_pressed(VirtualKeyCode::Key4) || input.key_held(VirtualKeyCode::Key4) {
            self.keys[3] = true;
        }
        if input.key_pressed(VirtualKeyCode::Q) || input.key_held(VirtualKeyCode::Q) {
            self.keys[4] = true;
        }
        if input.key_pressed(VirtualKeyCode::W) || input.key_held(VirtualKeyCode::W) {
            self.keys[5] = true;
        }
        if input.key_pressed(VirtualKeyCode::E) || input.key_held(VirtualKeyCode::E) {
            self.keys[6] = true;
        }
        if input.key_pressed(VirtualKeyCode::R) || input.key_held(VirtualKeyCode::R) {
            self.keys[7] = true;
        }
        if input.key_pressed(VirtualKeyCode::A) || input.key_held(VirtualKeyCode::A) {
            self.keys[8] = true;
        }
        if input.key_pressed(VirtualKeyCode::S) || input.key_held(VirtualKeyCode::S) {
            self.keys[9] = true;
        }
        if input.key_pressed(VirtualKeyCode::D) || input.key_held(VirtualKeyCode::D) {
            self.keys[10] = true;
        }
        if input.key_pressed(VirtualKeyCode::F) || input.key_held(VirtualKeyCode::F) {
            self.keys[11] = true;
        }
        if input.key_pressed(VirtualKeyCode::Z) || input.key_held(VirtualKeyCode::Z) {
            self.keys[12] = true;
        }
        if input.key_pressed(VirtualKeyCode::X) || input.key_held(VirtualKeyCode::X) {
            self.keys[13] = true;
        }
        if input.key_pressed(VirtualKeyCode::C) || input.key_held(VirtualKeyCode::C) {
            self.keys[14] = true;
        }
        if input.key_pressed(VirtualKeyCode::V) || input.key_held(VirtualKeyCode::V) {
            self.keys[15] = true;
        }

        if input.key_released(VirtualKeyCode::Key1) {
            self.keys[0] = false;
        }
        if input.key_released(VirtualKeyCode::Key2) {
            self.keys[1] = false;
        }
        if input.key_released(VirtualKeyCode::Key3) {
            self.keys[2] = false;
        }
        if input.key_released(VirtualKeyCode::Key4) {
            self.keys[3] = false;
        }
        if input.key_released(VirtualKeyCode::Q) {
            self.keys[4] = false;
        }
        if input.key_released(VirtualKeyCode::W) {
            self.keys[5] = false;
        }
        if input.key_released(VirtualKeyCode::E) {
            self.keys[6] = false;
        }
        if input.key_released(VirtualKeyCode::R) {
            self.keys[7] = false;
        }
        if input.key_released(VirtualKeyCode::A) {
            self.keys[8] = false;
        }
        if input.key_released(VirtualKeyCode::S) {
            self.keys[9] = false;
        }
        if input.key_released(VirtualKeyCode::D) {
            self.keys[10] = false;
        }
        if input.key_released(VirtualKeyCode::F) {
            self.keys[11] = false;
        }
        if input.key_released(VirtualKeyCode::Z) {
            self.keys[12] = false;
        }
        if input.key_released(VirtualKeyCode::X) {
            self.keys[13] = false;
        }
        if input.key_released(VirtualKeyCode::C) {
            self.keys[14] = false;
        }
        if input.key_released(VirtualKeyCode::V) {
            self.keys[15] = false;
        }

        if input.key_released(VirtualKeyCode::Escape) || input.quit() {
            *control_flow = ControlFlow::Exit;
            return;
        }
    }
}
