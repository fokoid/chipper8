use egui::{Context, Window};

use chipper8::instructions::Command;
use chipper8::machine::{self, Machine};
use windows::{Windowed, Index, Repl, Memory, Display, ExecutionStatus, Timers, Registers};

mod util;
mod windows;

pub struct Ui {
    windows: Vec<Box<dyn Windowed>>,
    repl: Repl,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            windows: vec![
                Box::new(Memory::new()),
                Box::new(Display::new()),
                Box::new(Registers::new()),
                Box::new(Index::new()),
                Box::new(ExecutionStatus::new()),
                Box::new(Timers::new()),
            ],
            // todo: move this inside windows list
            // the problem right now is that Repl::ui doesn't match the signature of the trait
            // Windowed::ui. This is because it takes a &mut reference to the command buffer. Once
            // the command input moves from this window to the bottom bar this issue will be fixed.
            repl: Repl::new(),
        }
    }

    pub fn add_history(&mut self, command: &Command, user: bool) {
        self.repl.add_history(command, user);
    }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine, command_buffer: &mut Option<Command>) {
        for window in &mut self.windows {
            Window::new(window.name())
                .resizable(false)
                .show(ctx, |ui| { window.ui(ui, machine); });
        }
        Window::new(self.repl.name()).show(ctx, |ui| { self.repl.ui(ui, command_buffer); });
      }
}