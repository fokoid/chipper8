use egui::{Response, TextStyle, Ui};
use egui::widgets::TextEdit;
use egui_extras::TableBuilder;
use ringbuffer::RingBufferExt;

use chipper8::instructions::Command;

use crate::ui::util::{MonoLabel, table, TabularData};
use crate::command_history::CommandHistory;

impl TabularData for &CommandHistory {
    fn header(&self) -> Option<Vec<MonoLabel>> {
        Some(vec![
            MonoLabel::new("Tag"),
            MonoLabel::new("Opcode"),
            MonoLabel::new("Command"),
            MonoLabel::new("Count"),
        ])
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        self.items.iter().map(|item| {
            vec![
                MonoLabel::new(
                    if item.command.is_meta() {
                        "M"
                    } else if item.user {
                        "U"
                    } else {
                        " "
                    },
                ),
                MonoLabel::new(match item.command.opcode() {
                    None => String::from(""),
                    Some(opcode) => format!("{}", opcode),
                },
                ),
                MonoLabel::new(format!("{}", item.command)),
                MonoLabel::new(
                    if item.count == 1 {
                        String::from("  ")
                    } else if item.count < 100
                    {
                        format!("{}", item.count)
                    } else { String::from("💯") },
                ),
            ]
        }).collect()
    }

    fn display_rows(&self) -> usize {
        16
    }
}

pub fn history_ui(ui: &mut Ui, history: &CommandHistory) {
    table::build(
        TableBuilder::new(ui)
            .resizable(false)
            .striped(true)
            .scroll(false)
            .stick_to_bottom(true),
        vec![30.0, 60.0, 160.0, 50.0],
        history,
    )
}

pub fn input_ui(ui: &mut Ui, text: &mut String) -> Response {
    ui.add(TextEdit::singleline(text)
        .font(TextStyle::Monospace)
        .frame(false)
        .hint_text(">>>")
        .desired_width(250.0))
}