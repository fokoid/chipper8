use egui::{Color32, Key, Label, Response, TextEdit, TextStyle, Ui};
use egui::widget_text::RichText;

use chipper8::instructions::{Command, MetaCommand};

use crate::State;

pub struct BottomBar {
    input: String,
}

impl BottomBar {
    pub fn new() -> Self {
        Self { input: String::new() }
    }

    pub fn ui(&mut self, ui: &mut Ui, state: &mut State) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut state.running, "Running");
            if ui.button("⏩").on_hover_text("Next Instruction").clicked() {
                state.command_buffer = Some(Command::Meta(MetaCommand::Step));
            }
            let response = input_ui(ui, &mut self.input);
            if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                state.parse_command(self.input.as_str());
                self.input.clear();
                response.request_focus();
            }
            if let Some(error) = state.error() {
                let message = RichText::new(format!("{}", error))
                    .color(Color32::DEBUG_COLOR);
                ui.add(Label::new(message));
            }
        });
    }
}

fn input_ui(ui: &mut Ui, text: &mut String) -> Response {
    ui.add(TextEdit::singleline(text)
        .font(TextStyle::Monospace)
        .frame(false)
        .hint_text(">>>")
        .desired_width(250.0))
}
