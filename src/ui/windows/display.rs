use egui::{RichText, Ui};

use crate::machine::{self, Machine};
use crate::ui::State;
use crate::ui::util::MemoryDisplay;

use super::WindowContent;

pub struct Display {
    display: MemoryDisplay,
    disable_hover_info: bool,
}

impl Display {
    pub fn new() -> Self {
        Self {
            display: MemoryDisplay::new(machine::config::DISPLAY_WIDTH, machine::config::DISPLAY_HEIGHT),
            disable_hover_info: false,
        }
    }

    pub fn minimal() -> Self {
        Self {
            display: MemoryDisplay::new(machine::config::DISPLAY_WIDTH, machine::config::DISPLAY_HEIGHT),
            disable_hover_info: true,
        }
    }

    // helper function to draw UI that does not require State since this widget doesn't need it and
    // it allows using this widget in the stateless basic emulator GUI
    pub fn ui_stateless(&mut self, ui: &mut Ui, machine: &Machine) {
        self.display.ui(ui, &machine.display, Vec::new(), |index| {
            if self.disable_hover_info { return vec![]; };
            let [x, y] = [index / machine::config::DISPLAY_WIDTH, index % machine::config::DISPLAY_WIDTH];
            let status = match machine.display.get(index) {
                Some(0xFF) => "ON",
                Some(0x00) => "OFF",
                _ => "UNKNOWN",
            };
            vec![RichText::new(format!("({}, {}): {}", x, y, status))]
        });
    }
}

impl WindowContent for Display {
    fn name(&self) -> &'static str { "Video Display" }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, _state: &mut State) {
        self.ui_stateless(ui, machine)
    }
}
