use egui::{Color32, Context, Frame, Stroke, Ui};
use egui::style::Margin;

use chipper8::machine::Machine;

use crate::ui::table::{self, TabularData};
use crate::ui::util::{self, MonoLabel};

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
    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        self.machine.stack.data.iter().enumerate().map(|(index, value)| {
            let mut label = MonoLabel::new(format!("{:04X}", value));
            vec![
                label.background_color(
                if index == self.machine.stack.pointer {
                    Some(Color32::LIGHT_RED)
                } else { None })
            ]
        }).collect()
    }
}

struct Other {}

impl Other {
    fn new() -> Self { Self {} }

    fn draw(&self, ui: &mut Ui, machine: &Machine) {
        ui.vertical(|ui| {
            ui.add(MonoLabel::new(format!("PC  {:04X} {:04X}", machine.program_counter, machine.at_program_counter())));
            if let Ok(instruction) = machine.next_instruction() {
                ui.add(MonoLabel::new(format!("{}", instruction)));
            };
            ui.add(MonoLabel::new(format!("IDX {:04X} {:04X}", machine.index, machine.at_index())));
            ui.add(MonoLabel::new(format!("DELAY {:02X}", machine.delay_timer)));
            ui.add(MonoLabel::new(format!("SOUND {:02X}", machine.sound_timer))
                .background_color(if machine.sound_timer > 0 { Some(Color32::LIGHT_RED) } else { None }));
        });
    }
}
