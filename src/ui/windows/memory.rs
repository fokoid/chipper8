use egui::Ui;

use chipper8::machine::Machine;

use crate::State;
use crate::ui::util::MemoryDisplay;

use super::WindowContent;

pub struct Memory {
    display: MemoryDisplay,
}

impl Memory {
    pub fn new() -> Self {
        Self { display: MemoryDisplay::new(64, 64) }
    }
}

impl WindowContent for Memory {
    fn name(&self) -> &'static str { "Memory" }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, _state: &mut State) {
        self.display.ui(ui, &machine.memory)
    }
}