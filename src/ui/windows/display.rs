use egui::Ui;

use chipper8::machine::{self, Machine};

use crate::ui::util::MemoryDisplay;

use super::Windowed;

pub struct Display {
    display: MemoryDisplay,
}

impl Display {
    pub fn new() -> Self {
        Self {
            display: MemoryDisplay::new(machine::DISPLAY_WIDTH, machine::DISPLAY_HEIGHT),
        }
    }
}

impl Windowed for Display {
    fn name(&self) -> &'static str { "Video Display" }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine) {
        self.display.ui(ui, &machine.display)
    }
}
