use egui::{Context, Window};

use chipper8::instructions::Command;
use chipper8::machine::{self, Machine};
use index::IndexDisplay;
use memory::MemoryDisplay;
use repl::Repl;

mod cpu;
mod memory;
mod repl;
mod util;
mod image_builder;
mod table;
mod index;
mod stack;
mod program_counter;

pub struct Ui {
    memory: MemoryDisplay,
    display: MemoryDisplay,
    repl: Repl,
    index: IndexDisplay,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            memory: MemoryDisplay::new(64, 64),
            display: MemoryDisplay::new(machine::DISPLAY_WIDTH, machine::DISPLAY_HEIGHT),
            repl: Repl::new(),
            index: IndexDisplay::new(),
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
            .show(ctx, |ui| cpu::registers_ui(ui, machine));
        Window::new("Execution Status").show(ctx, |ui| cpu::program_status_ui(ui, machine));
        Window::new("Timers").show(ctx, |ui| cpu::timers_ui(ui, machine));
        Window::new("Index").show(ctx, |ui| self.index.ui(ui, machine));
    }
}