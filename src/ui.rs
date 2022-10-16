use egui::{Align, Checkbox, Context, Layout, Sense};

use chipper8::machine::Machine;
use windows::Window;

use crate::State;

mod util;
mod windows;
mod bottom_bar;

pub struct Ui {
    windows: Vec<Window>,
    input: String,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            windows: windows::create_all(),
            input: String::new(),
        }
    }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine, state: &mut State) {
        egui::TopBottomPanel::bottom("bar").show(ctx, |ui| {
            bottom_bar::bottom_bar_ui(ui, &mut state.command_buffer, &mut self.input);
        });
        egui::CentralPanel::default().show(
            ctx, |_ui| {},
        ).response.interact(Sense::click()).context_menu(|ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.heading("Windows");
            });
            ui.separator();
            for window in &mut self.windows {
                let label_text = window.name();
                let checkbox = Checkbox::new(&mut window.open, label_text);
                if ui.add(checkbox).clicked() {
                    ui.close_menu();
                }
            }
        });
        for window in &mut self.windows {
            window.draw(ctx, machine, state);
        }
    }
}