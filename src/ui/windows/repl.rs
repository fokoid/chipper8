use egui::{Response, TextStyle, Ui};
use egui::widgets::TextEdit;
use egui_extras::TableBuilder;
use ringbuffer::RingBufferExt;

use crate::command_history::CommandHistory;
use crate::ui::util::{MonoLabel, table, TabularData};

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
