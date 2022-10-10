use egui::{Color32, Context, Frame, Label, Stroke, Ui};
use egui::style::Margin;

use chipper8::machine::Machine;

use crate::ui::table::{self, TabularData};
use crate::ui::util;

struct RegistersHelper<'a> {
    machine: &'a Machine,
}

impl<'a> RegistersHelper<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self { machine }
    }
}

impl<'a> TabularData for RegistersHelper<'a> {
    fn rows(&self) -> Vec<Vec<Label>> {
        self.machine.registers.iter().enumerate().map(|(index, value)| {
            vec![
                Label::new(util::monospace(&format!("V{:1X}", index))),
                Label::new(util::monospace(&format!("{:02X}", value))),
            ]
        }).collect()
    }
}

pub struct Cpu {
    other: Other,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            other: Other::new(),
        }
    }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine) {
        egui::SidePanel::right("vm-visualizer")
            .resizable(false)
            .min_width(230.0)
            .max_width(230.0)
            .frame(Frame::default()
                .inner_margin(Margin::symmetric(10.0, 5.0))
                .stroke(Stroke::new(2.0, Color32::DARK_GRAY)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.push_id(0, |ui| table::build(
                        ui,
                        vec![20.0, 20.0],
                        RegistersHelper::new(machine),
                    ));
                    ui.push_id(1, |ui| table::build(
                        ui,
                        vec![40.0],
                        StackHelper::new(machine),
                    ));
                    self.other.draw(ui, machine);
                });
            });
    }
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
    fn rows(&self) -> Vec<Vec<Label>> {
        self.machine.stack.data.iter().enumerate().map(|(index, value)| {
            let text = util::monospace(&format!("{:04X}", value));
            vec![
                Label::new(if index == self.machine.stack.pointer {
                    text.background_color(Color32::LIGHT_RED)
                } else { text })
            ]
        }).collect()
    }
}

struct Other {}

impl Other {
    fn new() -> Self { Self {} }

    fn draw(&self, ui: &mut Ui, machine: &Machine) {
        ui.vertical(|ui| {
            ui.label(util::monospace(&format!("PC  {:04X} {:04X}", machine.program_counter, machine.at_program_counter())));
            if let Ok(instruction) = machine.next_instruction() {
                ui.label(util::monospace(&format!("{}", instruction)));
            };
            ui.label(util::monospace(&format!("IDX {:04X} {:04X}", machine.index, machine.at_index())));
            ui.label(util::monospace(&format!("DELAY {:02X}", machine.delay_timer)));
            let sound_label = util::monospace(&format!("SOUND {:02X}", machine.sound_timer));
            ui.label(if machine.sound_timer > 0 { sound_label.background_color(Color32::LIGHT_RED) } else { sound_label });
        });
    }
}
