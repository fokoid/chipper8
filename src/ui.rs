mod cpu;
mod memory;
mod repl;
mod util;
mod image_builder;
mod table;

use egui::Context;
use cpu::Cpu;
use memory::Memory;
use repl::Repl;
use chipper8::instructions::{self, Command};
use chipper8::machine::Machine;

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

    pub fn draw(&mut self, ctx: &Context, machine: &Machine) -> instructions::Result<Option<Command>> {
        let result = self.repl.draw(ctx);
        self.cpu.draw(ctx, machine);
        self.memory.draw(ctx, machine);
        result
    }
}