use egui::Ui;
use egui_extras::TableBuilder;

use chipper8::machine::Machine;

use crate::ui::util::{MonoLabel, table, TabularData};

// todo: should we return a response?
pub fn stack_ui(ui: &mut Ui, machine: &Machine) {
    table::build(
        TableBuilder::new(ui)
            .striped(true)
            .stick_to_bottom(true)
            .resizable(false)
            .scroll(false),
        vec![50.0, 80.0, 50.0, 120.0],
        StackHelper::new(machine),
    )
}

struct StackHelper<'a> {
    machine: &'a Machine,
}

impl<'a> StackHelper<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self { machine }
    }
}

impl<'a> TabularData for StackHelper<'a> {
    fn header(&self) -> Option<Vec<MonoLabel>> {
        Some(vec![
            MonoLabel::new("Depth"),
            MonoLabel::new("Address"),
            MonoLabel::new("Value"),
            MonoLabel::new("Instruction"),
        ])
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        let mut rows: Vec<_> = self.machine.stack.data.iter().enumerate().map(|(index, address)| {
            let instruction = if let Ok(instruction) = self.machine.instruction_at_address(*address as usize) {
                format!("{}", instruction)
            } else {
                String::new()
            };
            vec![
                MonoLabel::new(if index == self.machine.stack.pointer {
                    format!(">{:01X}<", index)
                } else {
                    format!(" {:01X} ", index)
                }),
                MonoLabel::new(format!("{:03X}", address)),
                MonoLabel::new(format!("{:04X}", self.machine.word_at_address(*address as usize))),
                MonoLabel::new(instruction),
            ]
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