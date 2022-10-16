use egui::{Align, Context, Checkbox, Layout, Sense};

use chipper8::instructions::Command;
use chipper8::machine::Machine;
use windows::{Display, ExecutionStatus, Index, Memory, Registers, Timers, Window};
pub use windows::repl;

use crate::command_history::CommandHistory;

mod util;
mod windows;

pub struct Ui {
    windows: Vec<Window>,
    input: String,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            windows: vec![
                Window::new(Box::new(Display::new())),
                Window::new(Box::new(Memory::new())),
                Window::new(Box::new(Registers::new())),
                Window::new(Box::new(Index::new())),
                Window::new(Box::new(Timers::new())),
                Window::new(Box::new(ExecutionStatus::new())),
            ],
            input: String::new(),
        }
    }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine, command_buffer: &mut Option<Command>, history: &CommandHistory) {
        egui::TopBottomPanel::bottom("bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if repl::input_ui(ui, &mut self.input).lost_focus() {
                    match Command::parse(self.input.as_str().into()) {
                        Ok(Some(command)) => {
                            self.input.clear();
                            command_buffer.replace(command);
                        }
                        Ok(None) => {}
                        Err(error) => {
                            eprintln!("{:?}", error);
                        }
                    };
                };
            });
        });
        egui::CentralPanel::default().show(
            ctx, |_ui| {}
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
        egui::Window::new("Command History")
            .resizable(false)
            .show(ctx, |ui| { repl::history_ui(ui, history) });
        for window in &mut self.windows {
            window.draw(ctx, machine);
        }
    }
}