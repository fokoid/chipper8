use egui::{Context, Window};

use chipper8::instructions::Command;
use chipper8::machine::{self, Machine};
use util::MemoryDisplay;
use windows::{Index, Repl};

mod util;
mod windows;

pub struct Ui {
    memory: MemoryDisplay,
    display: MemoryDisplay,
    repl: Repl,
    index: Index,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            memory: MemoryDisplay::new(64, 64),
            display: MemoryDisplay::new(machine::DISPLAY_WIDTH, machine::DISPLAY_HEIGHT),
            repl: Repl::new(),
            index: Index::new(),
        }
    }

    pub fn add_history(&mut self, command: &Command, user: bool) {
        self.repl.add_history(command, user);
    }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine, command_buffer: &mut Option<Command>) {
        Window::new("REPL").show(ctx, |ui| { self.repl.ui(ui, command_buffer); });
        Window::new("Display").show(ctx, |ui| self.display.ui(ui, &machine.display));
        Window::new("Memory").show(ctx, |ui| self.memory.ui(ui, &machine.memory));
        Window::new("Registers")
            .default_size([100.0, 600.0])
            .resizable(false)
            .show(ctx, |ui| windows::registers_ui(ui, machine));
        Window::new("Execution Status").show(ctx, |ui| windows::execution_status_ui(ui, machine));
        Window::new("Timers").show(ctx, |ui| windows::timers_ui(ui, machine));
        Window::new("Index").show(ctx, |ui| self.index.ui(ui, machine));
    }
}