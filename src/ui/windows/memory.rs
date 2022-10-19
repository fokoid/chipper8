use egui::{Color32, Ui};
use egui::widget_text::RichText;

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

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, state: &mut State) {
        self.display.image_builder.color_map.fill(Color32::WHITE);
        for (tag, range) in state.memory_tags.iter() {
            self.display.image_builder.color_map[range.clone()].fill(tag.color());
        }
        self.display.ui(ui, &machine.memory, vec![machine.program_counter, machine.program_counter + 1, machine.index]);
        for tag in state.memory_tags.keys() {
            ui.label(RichText::new(tag.name())
                .color(tag.color()));
        }
    }
}