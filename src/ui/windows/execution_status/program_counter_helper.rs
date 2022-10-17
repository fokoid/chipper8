use chipper8::machine::Machine;

use crate::ui::util::{MonoLabel, TabularData};

use super::shared;

pub struct ProgramCounterHelper<'a> {
    pub machine: &'a Machine,
}

impl<'a> TabularData for ProgramCounterHelper<'a> {
    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        vec![shared::address_row("", self.machine.program_counter, self.machine)]
    }
}