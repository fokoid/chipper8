use egui::Ui;
use egui_extras::TableBuilder;

use chipper8::machine::Machine;

use crate::ui::util::{MonoLabel, table, TabularData};

// todo: should we return a response?
pub fn registers_ui(ui: &mut Ui, machine: &Machine) {
    table::build(
        TableBuilder::new(ui)
            .striped(true)
            .stick_to_bottom(true)
            .resizable(false)
            .scroll(false)
        // .column(Size::relative(0.5))
        // .column(Size::relative(0.5))
        ,
        vec![60.0, 60.0, 60.0],
        RegistersHelper::new(machine),
    )
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
    fn header(&self) -> Option<Vec<MonoLabel>> {
        Some(vec![
            MonoLabel::new("Name"),
            MonoLabel::new("Hex"),
            MonoLabel::new("Decimal"),
        ])
    }

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
