use egui::{TextStyle, Ui, WidgetText};

use chipper8::machine::Machine;

use crate::State;
use crate::ui::util::{Byte, Decimal, Register, TabularData};
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
            ).striped(true)
        }
    }
}

impl WindowContent for Registers {
    fn name(&self) -> &'static str {
        "Registers"
    }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, _state: &mut State) {
        ui.style_mut().override_text_style = Some(TextStyle::Monospace);
        self.table_spec.draw(ui, RegistersHelper::new(machine))
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
    fn rows(&self) -> Vec<Vec<WidgetText>> {
        self.machine.registers.iter().enumerate().map(|(index, value)| {
            vec![
                Register::from(index).into(),
                Byte::from(*value).into(),
                Decimal::from(*value).into(),
            ]
        }).collect()
    }
}
