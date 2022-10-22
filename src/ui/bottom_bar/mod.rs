use egui::{Color32, Label, Slider, Ui};
use egui::widget_text::RichText;

use input::Input;

use crate::command::{Command, MetaCommand};
use crate::machine::Machine;
use crate::ui::State;
use crate::ui::util::Nibble;
use crate::ui::util::table::TabularData;
use crate::ui::windows::ProgramCounterHelper;

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
            ui.label("Machine Tick Rate: ");
            ui.add(Slider::new(&mut state.frames_per_second, 1..=120));
            ui.checkbox(&mut state.running, "Running");
            ui.checkbox(&mut state.skip_unknown_opcode, "Skip Unknown Opcode");
            if ui.button("⏩").on_hover_text("Next Instruction").clicked() {
                state.command_buffer = Some(Command::Meta(MetaCommand::Tick));
            }
            ui.separator();
            self.input.ui(ui, state);
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
                ui.label(label.monospace());
            }
            ui.separator();
            ui.label(format!("ROM: {}", state.rom.as_ref().map_or("none", |rom| &rom.name)));
            ui.separator();
            let key_buffer = machine.key_buffer.map(|key| String::from(Nibble::from(key)));
            ui.label(format!("Key buffer: {}", key_buffer.unwrap_or(String::from("none"))));
        });
    }
}