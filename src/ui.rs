use egui::{Context, Window};

use chipper8::instructions::Command;
use chipper8::machine::Machine;
use windows::{Display, ExecutionStatus, Index, Memory, Registers, Timers, Windowed};
pub use windows::repl;

use crate::command_history::CommandHistory;

mod util;
mod windows;

pub struct Ui {
    windows: Vec<Box<dyn Windowed>>,
    input: String,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            windows: vec![
                Box::new(Display::new()),
                Box::new(Memory::new()),
                Box::new(Registers::new()),
                Box::new(Index::new()),
                Box::new(Timers::new()),
                Box::new(ExecutionStatus::new()),
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
        Window::new("Command History")
            .resizable(false)
            .show(ctx, |ui| { repl::history_ui(ui, history) });
        for window in &mut self.windows {
            Window::new(window.name())
                .resizable(false)
                .show(ctx, |ui| { window.ui(ui, machine); });
        }
    }
}