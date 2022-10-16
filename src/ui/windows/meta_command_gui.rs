use egui::{TextEdit, Ui};

use chipper8::machine::Machine;

use crate::State;

use super::WindowContent;

pub struct MetaCommandGui {
    load_filename: String,
    load_address: String,
}

impl MetaCommandGui {
    pub fn new() -> Self {
        Self {
            load_filename: String::new(),
            load_address: String::new(),
        }
    }
}

impl WindowContent for MetaCommandGui {
    fn name(&self) -> &'static str { "Meta Commands" }

    // todo: this is currently very hard coded
    fn ui(&mut self, ui: &mut Ui, _machine: &Machine, state: &mut State) {
        ui.horizontal(|ui| {
            for (label, command) in vec![
                ("Play", ".play"),
                ("Pause", ".pause"),
                ("Play / Pause", ".play-pause"),
                ("Step", ".step"),
            ] {
                if ui.button(label).clicked() {
                    state.parse_command(command);
                }
            }
        });
        ui.horizontal(|ui| {
            for (label, machine_state) in vec![
                ("Init", ""),
                ("Demo", "demo"),
            ] {
                if ui.button(format!("Reset ({})", label)).clicked() {
                    state.parse_command(&format!(".reset {}", machine_state));
                }
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Load").clicked() {
                state.parse_command(&format!(".load {} {}", self.load_filename, self.load_address));
            }
            ui.add(TextEdit::singleline(&mut self.load_filename)
                .hint_text("ROM filename")
                .desired_width(100.0));
            ui.add(TextEdit::singleline(&mut self.load_address)
                .hint_text("Address")
                .desired_width(50.0));
        });
    }
}