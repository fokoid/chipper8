use egui::{Align, Layout, Ui};

use chipper8::machine::Machine;

use super::WindowContent;

mod program_counter;
mod stack;
mod shared;

pub struct ExecutionStatus {}

impl ExecutionStatus {
    pub fn new() -> Self { Self {} }
}

impl WindowContent for ExecutionStatus {
    fn name(&self) -> &'static str {
        "Execution Status"
    }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine) {
        ui.push_id(0, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.heading("Program Counter")
            });
            program_counter::program_counter_ui(ui, machine);
        });
        ui.add_space(18.0);
        ui.push_id(1, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.heading("Stack")
            });
            stack::stack_ui(ui, machine);
        });
    }
}