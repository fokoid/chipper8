use egui::Ui;
use egui_extras::TableBuilder;

use chipper8::machine::Machine;

use crate::State;
use crate::ui::util::{MonoLabel, TabularData};
use crate::ui::util::table::{ColumnSpec, TableSpec};

use super::WindowContent;

pub struct Registers {
    table_spec: TableSpec,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            table_spec: TableSpec::new(
                vec![
                    ColumnSpec::fixed("Name", 60.0),
                    ColumnSpec::fixed("Hex", 60.0),
                    ColumnSpec::fixed("Decimal", 60.0),
                ],
            )
        }
    }
}

impl WindowContent for Registers {
    fn name(&self) -> &'static str {
        "Registers"
    }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, _state: &mut State) {
        self.table_spec.build(
            TableBuilder::new(ui)
                .striped(true)
                .stick_to_bottom(true)
                .resizable(false)
                .scroll(false)
            // .column(Size::relative(0.5))
            // .column(Size::relative(0.5))
            ,
            RegistersHelper::new(machine),
        )
    }
}

struct RegistersHelper<'a> {
    machine: &'a Machine,
}

impl<'a> RegistersHelper<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self { machine }
    }
}

impl<'a> TabularData for RegistersHelper<'a> {
    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        self.machine.registers.iter().enumerate().map(|(index, value)| {
            vec![
                MonoLabel::new(format!("V{:1X}", index)),
                MonoLabel::new(format!("{:02X}", value)),
                MonoLabel::new(format!("{:03}", value)),
            ]
        }).collect()
    }
}
