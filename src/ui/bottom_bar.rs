use egui::{Color32, Key, Label, Response, TextEdit, TextStyle, Ui};
use egui::widget_text::RichText;
use ringbuffer::RingBuffer;

use chipper8::instructions::{Command, MetaCommand};
use chipper8::machine::Machine;

use crate::State;
use crate::ui::util::table::TabularData;
use crate::ui::windows::ProgramCounterHelper;
use input::Input;

mod input;

pub struct BottomBar {
    input: Input,
}

impl BottomBar {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
        }
    }

    // todo: use strips (or something else) to force some of this content to the right
    pub fn ui(&mut self, ui: &mut Ui, machine: &Machine, state: &mut State) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut state.running, "Running");
            if ui.button("⏩").on_hover_text("Next Instruction").clicked() {
                state.command_buffer = Some(Command::Meta(MetaCommand::Step));
            }
            ui.separator();
            let response = self.input.ui(ui, state);
            if let Some(error) = state.error() {
                ui.separator();
                let message = RichText::new(format!("{}", error))
                    .color(Color32::DEBUG_COLOR);
                ui.add(Label::new(message));
            }
            ui.separator();
            ui.label("Program Counter");
            // todo: dependent on table UI implementation, subject to change
            let helper = ProgramCounterHelper { machine };
            for label in helper.rows().into_iter().next().unwrap() {
                ui.add(label);
            }
        });
    }


}