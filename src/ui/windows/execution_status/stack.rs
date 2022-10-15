use egui::Ui;

use chipper8::machine::Machine;

use crate::ui::util::{MonoLabel, TabularData};

use super::shared;

// todo: should we return a response?
pub fn stack_ui(ui: &mut Ui, machine: &Machine) {
    shared::address_table(ui, StackHelper { machine })
}

struct StackHelper<'a> {
    pub machine: &'a Machine,
}

impl<'a> TabularData for StackHelper<'a> {
    fn header(&self) -> Option<Vec<MonoLabel>> {
        Some(shared::header_row("Depth"))
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        let mut rows: Vec<_> = self.machine.stack.data.iter().enumerate().map(|(index, address)| {
            let prefix = if index == self.machine.stack.pointer {
                format!(">{:01X}<", index)
            } else {
                format!(" {:01X} ", index)
            };
            shared::address_row(&prefix, *address as usize, self.machine)
        }).collect();
        rows.push(vec![
            MonoLabel::new(if 16 == self.machine.stack.pointer { "> <" } else { "   " }),
            MonoLabel::new(""),
            MonoLabel::new(""),
            MonoLabel::new(""),
        ]);
        rows
    }
}