use super::display;
use super::kb;
use super::ram;

pub struct DataBus {
    pub ram: ram::Ram,
    pub display: display::Display,
    pub keyboard: kb::Keyboard,
}

impl DataBus {
    pub fn new() -> Self {
        Self {
            ram: ram::Ram::new(),
            display: display::Display::new(),
            keyboard: kb::Keyboard::new(),
        }
    }
}
