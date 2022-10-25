use egui::{Color32, RichText, Ui};

use crate::machine::{Machine, types};
use crate::ui::State;
use crate::ui::util::{Address, Byte, Word};
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
        self.display.ui(ui,
                        &machine.memory,
                        vec![usize::from(&machine.program_counter),
                             usize::from(&machine.program_counter) + 1,
                             usize::from(&machine.index)],
                        |index| hover_text(index, machine, state));
    }
}

fn hover_text(index: usize, machine: &Machine, state: &State) -> Vec<RichText> {
    // todo: don't panic here
    let address = types::Address::try_from(index).expect("index exceeds allowed machine memory size");
    let mut lines = vec![
        RichText::new(format!("At memory offset 0x{}:", Address::from(index))),
    ];
    if let Some(word) = machine.word_at_address(&address) {
        lines.push(RichText::new(format!(" · Word {}", Word::from(word))));
    } else if let Some(byte) = machine.byte_at_address(&address) {
        lines.push(RichText::new(format!(" · Byte {}", Byte::from(byte))));
    }
    if let Ok(instruction) = machine.instruction_at_address(&address) {
        lines.push(RichText::new(format!(" · Instruction: {}", instruction)));
    }
    for (tag, range) in &state.memory_tags {
        if range.contains(&index) {
            lines.push(RichText::from(format!(" · {}", tag.name())).color(tag.color()))
        }
    };
    lines
}