use chipper8::machine::Machine;
use egui::Ui;
use egui_extras::TableBuilder;
use ringbuffer::RingBufferExt;

use crate::State;
use crate::ui::util::{MonoLabel, table, TabularData};

use super::WindowContent;

pub struct CommandHistory {}

impl CommandHistory {
    pub fn new() -> Self { Self {} }
}

impl WindowContent for CommandHistory {
    fn name(&self) -> &'static str { "Command History" }

    fn ui(&mut self, ui: &mut Ui, _machine: &Machine, state: &mut State) {
        table::build(
            TableBuilder::new(ui)
                .resizable(false)
                .striped(true)
                .scroll(false)
                .stick_to_bottom(true),
            vec![30.0, 60.0, 160.0, 50.0],
            &state.command_history,
        )
    }
}

impl TabularData for &crate::command_history::CommandHistory {
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
