use egui::Ui;
use egui_extras::TableBuilder;
use ringbuffer::RingBufferExt;

use chipper8::machine::Machine;

use crate::State;
use crate::ui::util::{MonoLabel, TabularData};
use crate::ui::util::table::{ColumnSpec, TableSpec};

use super::WindowContent;

pub struct CommandHistory {
    table_spec: TableSpec,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            table_spec: TableSpec::new(vec![
                ColumnSpec::fixed("Tag", 30.0),
                ColumnSpec::fixed("Opcode", 60.0),
                ColumnSpec::fixed("Command", 160.0),
                ColumnSpec::fixed("Count", 50.0),
            ])
        }
    }
}

impl WindowContent for CommandHistory {
    fn name(&self) -> &'static str { "Command History" }

    fn ui(&mut self, ui: &mut Ui, _machine: &Machine, state: &mut State) {
        self.table_spec.build(
            TableBuilder::new(ui)
                .resizable(false)
                .striped(true)
                .scroll(false)
                .stick_to_bottom(true),
            &state.command_history,
        )
    }
}

impl TabularData for &crate::command_history::CommandHistory {
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
