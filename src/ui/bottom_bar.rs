use egui::{Response, TextEdit, TextStyle, Ui};
use chipper8::instructions::Command;

pub fn bottom_bar_ui(ui: &mut Ui, command_buffer: &mut Option<Command>, input: &mut String) {
    ui.horizontal(|ui| {
        if input_ui(ui, input).lost_focus() {
            match Command::parse(input.as_str().into()) {
                Ok(Some(command)) => {
                    input.clear();
                    command_buffer.replace(command);
                }
                Ok(None) => {}
                Err(error) => {
                    eprintln!("{:?}", error);
                }
            };
        };
    });
}

fn input_ui(ui: &mut Ui, text: &mut String) -> Response {
    ui.add(TextEdit::singleline(text)
        .font(TextStyle::Monospace)
        .frame(false)
        .hint_text(">>>")
        .desired_width(250.0))
}
