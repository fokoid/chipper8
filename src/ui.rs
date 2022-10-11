use egui::{Context, Response};

use chipper8::instructions::Command;
use chipper8::machine::Machine;
use cpu::Cpu;
use memory::Memory;
use repl::Repl;

mod cpu;
mod memory;
mod repl;
mod util;
mod image_builder;
mod table;

pub struct Ui {
    cpu: Cpu,
    memory: Memory,
    repl: Repl,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            repl: Repl::new(),
        }
    }

    pub fn add_history(&mut self, command: &Command, user: bool) {
        self.repl.add_history(command, user);
    }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine, command_buffer: &mut Option<Command>) -> Response {
        let response = self.repl.ui(ctx, command_buffer);
        self.cpu.draw(ctx, machine);
        self.memory.draw(ctx, machine);
        response
    }
}