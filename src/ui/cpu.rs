use egui::{Color32, Context, Frame, Response, Stroke, Ui};
use egui::style::Margin;

use chipper8::machine::Machine;

use crate::ui::table::{self, TabularData};
use crate::ui::util::MonoLabel;

// todo: should we return a response?
pub fn registers_ui(ui: &mut Ui, machine: &Machine) {
    table::build(
        ui,
        vec![20.0, 20.0],
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
    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        self.machine.registers.iter().enumerate().map(|(index, value)| {
            vec![
                MonoLabel::new(format!("V{:1X}", index)),
                MonoLabel::new(format!("{:02X}", value)),
            ]
        }).collect()
    }
}

// todo: should we return a response?
pub fn stack_ui(ui: &mut Ui, machine: &Machine) {
    table::build(
        ui,
        vec![40.0],
        StackHelper::new(machine),
    )
}

struct StackHelper<'a> {
    machine: &'a Machine,
}

impl<'a> StackHelper<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self { machine }
    }
}

impl<'a> TabularData for StackHelper<'a> {
    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        self.machine.stack.data.iter().enumerate().map(|(index, value)| {
            let label = MonoLabel::new(format!("{:04X}", value));
            vec![
                label.highlight_if(|| index == self.machine.stack.pointer)
            ]
        }).collect()
    }
}

pub fn pointers_ui(ui: &mut Ui, machine: &Machine) -> Response {
    ui.vertical(|ui| {
        ui.add(MonoLabel::new(format!("PC  {:04X} {:04X}", machine.program_counter, machine.at_program_counter())));
        if let Ok(instruction) = machine.next_instruction() {
            ui.add(MonoLabel::new(format!("{}", instruction)));
        };
        ui.add(MonoLabel::new(format!("IDX {:04X} {:04X}", machine.index, machine.at_index())));
    }).response
}

pub fn timers_ui(ui: &mut Ui, machine: &Machine) -> Response {
    ui.vertical(|ui| {
        ui.add(MonoLabel::new(format!("DELAY {:02X}", machine.delay_timer)));
        ui.add(MonoLabel::new(format!("SOUND {:02X}", machine.sound_timer))
            .highlight_if(|| machine.sound_timer > 0));
    }).response
}