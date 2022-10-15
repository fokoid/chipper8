use egui::Ui;
use egui_extras::TableBuilder;

use chipper8::machine::Machine;

use crate::ui::util::{table, TabularData};
use crate::ui::util::MonoLabel;

pub fn header_row(prefix: &str) -> Vec<MonoLabel> {
    vec![
        MonoLabel::new(prefix),
        MonoLabel::new("Address"),
        MonoLabel::new("Value"),
        MonoLabel::new("Instruction"),
    ]
}

pub fn address_row(prefix: &str, address: usize, machine: &Machine) -> Vec<MonoLabel> {
    let instruction = if let Ok(instruction) = machine.instruction_at_address(address) {
        format!("{}", instruction)
    } else {
        String::new()
    };
    vec![
        MonoLabel::new(prefix),
        MonoLabel::new(format!("{:03X}", address)),
        MonoLabel::new(format!("{:04X}", machine.word_at_address(address))),
        MonoLabel::new(instruction),
    ]
}

pub fn address_table<Helper>(ui: &mut Ui, helper: Helper)
    where Helper: TabularData {
    table::build(
        TableBuilder::new(ui)
            .striped(true)
            .stick_to_bottom(true)
            .resizable(false)
            .scroll(false),
        vec![50.0, 80.0, 50.0, 120.0],
        helper,
    )
}