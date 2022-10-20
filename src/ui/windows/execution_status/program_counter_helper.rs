use egui::WidgetText;

use chipper8::machine::Machine;

use crate::ui::util::TabularData;

use super::shared;

pub struct ProgramCounterHelper<'a> {
    pub machine: &'a Machine,
}

impl<'a> TabularData for ProgramCounterHelper<'a> {
    fn rows(&self) -> Vec<Vec<WidgetText>> {
        vec![shared::address_row("", self.machine.program_counter, self.machine)]
    }
}