use egui::Ui;
use egui_extras::TableBuilder;

use chipper8::machine::Machine;

use crate::ui::util::{MonoLabel, table, TabularData};

// todo: should we return a response?
pub fn program_counter_ui(ui: &mut Ui, machine: &Machine) {
    table::build(
        TableBuilder::new(ui)
            .resizable(false)
            .scroll(false),
        vec![50.0, 80.0, 50.0, 120.0],
        ProgramCounterHelper::new(machine),
    )
}

struct ProgramCounterHelper<'a> {
    machine: &'a Machine,
}

impl<'a> ProgramCounterHelper<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self { machine }
    }
}

impl<'a> TabularData for ProgramCounterHelper<'a> {
    fn header(&self) -> Option<Vec<MonoLabel>> {
        Some(vec![
            MonoLabel::new("", ),
            MonoLabel::new("Address", ),
            MonoLabel::new("Value", ),
            MonoLabel::new("Instruction", ),
        ])
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        let instruction = if let Ok(instruction) = self.machine.next_instruction() {
            format!("{}", instruction)
        } else {
            String::new()
        };
        vec![
            vec![
                MonoLabel::new(""),
                MonoLabel::new(format!("{:03X}", self.machine.program_counter)),
                MonoLabel::new(format!("{:04X}", self.machine.at_program_counter())),
                MonoLabel::new(instruction),
            ],
        ]
    }
}