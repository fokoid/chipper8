use egui::Ui;

use chipper8::machine::Machine;

use crate::ui::util::{MonoLabel, TabularData};

use super::shared;

// todo: should we return a response?
pub fn program_counter_ui(ui: &mut Ui, machine: &Machine) {
    shared::address_table(ui, ProgramCounterHelper { machine })
}

pub struct ProgramCounterHelper<'a> {
    pub machine: &'a Machine,
}

impl<'a> TabularData for ProgramCounterHelper<'a> {
    fn header(&self) -> Option<Vec<MonoLabel>> {
        Some(shared::header_row(""))
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        vec![shared::address_row("", self.machine.program_counter, self.machine)]
    }
}