use egui::{Ui, TextStyle, WidgetText};
use ringbuffer::RingBufferExt;

use chipper8::machine::Machine;

use crate::State;
use crate::ui::util::TabularData;
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
            ]).striped(true)
        }
    }
}

impl WindowContent for CommandHistory {
    fn name(&self) -> &'static str { "Command History" }

    fn ui(&mut self, ui: &mut Ui, _machine: &Machine, state: &mut State) {
        ui.style_mut().override_text_style = Some(TextStyle::Monospace);
        self.table_spec.draw(ui, &state.command_history)
    }
}

impl TabularData for &crate::command_history::CommandHistory {
    fn rows(&self) -> Vec<Vec<WidgetText>> {
        self.items.iter().map(|item| {
            vec![
                String::from(
                    if item.command.is_meta() {
                        "M"
                    } else if item.user {
                        "U"
                    } else {
                        " "
                    },
                ).into(),
                match item.command.opcode() {
                    None => String::from(""),
                    Some(opcode) => format!("{}", opcode),
                }.into(),
                format!("{}", item.command).into(),
                if item.count == 1 {
                    String::from("  ")
                } else if item.count < 100
                {
                    format!("{}", item.count)
                } else { String::from("💯") }.into(),
            ]
        }).collect()
    }

    fn display_rows(&self) -> usize {
        16
    }
}
