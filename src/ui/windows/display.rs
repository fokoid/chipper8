use egui::{RichText, Ui};

use chipper8::machine::{self, Machine};

use crate::State;
use crate::ui::util::MemoryDisplay;

use super::WindowContent;

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

impl WindowContent for Display {
    fn name(&self) -> &'static str { "Video Display" }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, _state: &mut State) {
        self.display.ui(ui, &machine.display, Vec::new(), |index| {
            let [x, y] = [index / machine::DISPLAY_WIDTH, index % machine::DISPLAY_WIDTH];
            let status = match machine.display.get(index) {
                Some(0xFF) => "ON",
                Some(0x00) => "OFF",
                _ => "UNKNOWN",
            };
            vec![RichText::new(format!("({}, {}): {}", x, y, status))]
        });
    }
}
