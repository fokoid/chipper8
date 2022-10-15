use chipper8::machine::Machine;
use egui::Ui;

use crate::ui::util::MemoryDisplay;

use super::Windowed;

pub struct Memory {
    display: MemoryDisplay,
}

impl Memory {
    pub fn new() -> Self {
        Self { display: MemoryDisplay::new(64, 64) }
    }
}

impl Windowed for Memory {
    fn name(&self) -> &'static str { "Memory" }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine) {
        self.display.ui(ui, &machine.memory)
    }
}