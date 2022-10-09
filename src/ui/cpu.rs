use egui::{Color32, Context, Frame, Stroke};
use egui::style::Margin;
use egui_extras::{Size, TableBuilder};
use crate::ui::util;
use chipper8::machine::Machine;

pub struct Cpu {}

impl Cpu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &Context, machine: &Machine) {
        egui::SidePanel::right("vm-visualizer")
            .resizable(false)
            .min_width(230.0)
            .max_width(230.0)
            .frame(Frame::default()
                .inner_margin(Margin::symmetric(10.0, 5.0))
                .stroke(Stroke::new(2.0, Color32::DARK_GRAY)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.push_id(0, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .column(Size::exact(20.0))
                            .column(Size::exact(20.0))
                            .resizable(false)
                            .scroll(false)
                            .body(|mut body| {
                                for (index, value) in machine.registers.iter().enumerate() {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(util::monospace(&format!("V{:1X}", index)));
                                        });
                                        row.col(|ui| { ui.label(util::monospace(&format!("{:02X}", value))); });
                                    });
                                };
                            });
                    });
                    ui.push_id(1, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .column(Size::exact(40.0))
                            .resizable(false)
                            .scroll(false)
                            .body(|mut body| {
                                for index in 0..16 {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            let text = util::monospace(&format!("{:04X}", machine.stack.data[index]));
                                            ui.label(
                                                if index == machine.stack.pointer {
                                                    text.background_color(Color32::LIGHT_RED)
                                                } else { text }
                                            );
                                        });
                                    });
                                }
                            });
                    });
                    ui.vertical(|ui| {
                        ui.label(util::monospace(&format!("PC  {:04X} {:04X}", machine.program_counter, machine.at_program_counter())));
                        if let Ok(instruction) = machine.next_instruction() {
                            ui.label(util::monospace(&format!("{}", instruction)));
                        };
                        ui.label(util::monospace(&format!("IDX {:04X} {:04X}", machine.index, machine.at_index())));
                        ui.label(util::monospace(&format!("DELAY {:02X}", machine.delay_timer)));
                        let sound_label = util::monospace(&format!("SOUND {:02X}", machine.sound_timer));
                        ui.label(if machine.sound_timer > 0 { sound_label.background_color(Color32::LIGHT_RED) } else { sound_label });
                    })
                });
            });
    }
}