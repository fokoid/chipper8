use egui::{Align, Checkbox, Context, Layout, Sense};

use bottom_bar::BottomBar;
pub use command_history::CommandHistory;
pub use state::{MemoryTag, Rom, State};
use windows::Window;

use crate::machine::Machine;

mod util;
mod windows;
mod bottom_bar;
mod state;
mod command_history;

pub struct Ui {
    windows: Vec<Window>,
    bottom_bar: BottomBar,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            windows: windows::create_all(),
            bottom_bar: BottomBar::new(),
        }
    }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine, state: &mut State) {
        egui::TopBottomPanel::bottom("bar").show(ctx, |ui| {
            self.bottom_bar.ui(ui, machine, state);
        });
        egui::CentralPanel::default().show(
            ctx, |_ui| {},
        ).response.interact(Sense::click()).context_menu(|ui| {
            window_menu_ui(ui, &mut self.windows)
        });
        for window in &mut self.windows {
            window.draw(ctx, machine, state);
        }
    }
}

fn window_menu_ui(ui: &mut egui::Ui, windows: &mut Vec<Window>) {
    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        ui.heading("Windows");
    });
    ui.separator();
    for window in windows {
        let label_text = window.name();
        let checkbox = Checkbox::new(&mut window.open, label_text);
        if ui.add(checkbox).clicked() {
            ui.close_menu();
        }
    }
    ui.separator();
    if ui.button("Auto-arrange").clicked() {
        ui.ctx().memory().reset_areas();
        ui.close_menu();
    }
}