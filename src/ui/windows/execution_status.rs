use egui::{Align, Layout, Ui};

use chipper8::machine::Machine;
// this is pub so the bottom bar can also use it to create a program counter representation
pub use program_counter_helper::ProgramCounterHelper;
use shared::AddressTable;
use stack_helper::StackHelper;

use crate::State;

use super::WindowContent;

mod program_counter_helper;
mod stack_helper;
mod shared;

pub struct ExecutionStatus {
    program_counter: AddressTable,
    stack: AddressTable,
}

impl ExecutionStatus {
    pub fn new() -> Self {
        Self {
            program_counter: AddressTable::new(""),
            stack: AddressTable::new("Depth"),
        }
    }
}

impl WindowContent for ExecutionStatus {
    fn name(&self) -> &'static str {
        "Execution Status"
    }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, _state: &mut State) {
        ui.push_id(0, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.heading("Program Counter")
            });
            self.program_counter.ui(ui, ProgramCounterHelper { machine });
        });
        ui.add_space(18.0);
        ui.push_id(1, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.heading("Stack")
            });
            self.stack.ui(ui, StackHelper { machine })
        });
    }
}