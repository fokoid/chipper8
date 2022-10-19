use egui::{Color32, Ui};
use egui::widget_text::RichText;
use itertools::Itertools;

use chipper8::machine::Machine;

use crate::{MemoryTag, State};
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
        for (tag, buffer) in state.memory_tags.iter().zip(self.display.image_builder.color_map.iter_mut()) {
            *buffer = tag.as_ref().map_or(Color32::WHITE, MemoryTag::color);
        }
        self.display.ui(ui, &machine.memory);
        let unique_tags: Vec<_> = state.memory_tags.iter().unique().collect();
        ui.horizontal(|ui| {
            for tag in unique_tags.into_iter() {
                if let Some(tag) = tag {
                    ui.label(RichText::new(tag.name())
                        .color(tag.color()));
                }
            }
        });
    }
}