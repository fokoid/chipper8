use egui::Ui;

use chipper8::machine::Machine;

use crate::ui::util::{MonoLabel, Address, Word};
use crate::ui::util::table::{ColumnSpec, TableSpec, TabularData};

pub fn address_row(prefix: &str, address: usize, machine: &Machine) -> Vec<MonoLabel> {
    let instruction = if let Ok(instruction) = machine.instruction_at_address(address) {
        format!("{}", instruction)
    } else {
        String::new()
    };
    vec![
        MonoLabel::new(prefix),
        MonoLabel::new(Address::from(address)),
        MonoLabel::new(Word::from(machine.word_at_address(address))),
        MonoLabel::new(instruction),
    ]
}

pub struct AddressTable {
    table_spec: TableSpec,
}

impl AddressTable {
    pub fn new(prefix: &str) -> Self {
        Self {
            table_spec: TableSpec::new(
                vec![
                    ColumnSpec::fixed(prefix, 50.0),
                    ColumnSpec::fixed("Address", 80.0),
                    ColumnSpec::fixed("Value", 50.0),
                    ColumnSpec::fixed("Instruction", 120.0),
                ]
            ).striped(true)
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, helper: impl TabularData) {
        self.table_spec.draw(ui, helper)
    }
}