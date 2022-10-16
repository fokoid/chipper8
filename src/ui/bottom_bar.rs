use egui::{Key, Response, TextEdit, TextStyle, Ui};
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
            let response = input_ui(ui, &mut self.input);
            if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                state.parse_command(self.input.as_str());
                self.input.clear();
            }
            response.request_focus();
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
