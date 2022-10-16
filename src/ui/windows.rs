use egui::{Context, Ui};

use chipper8::machine::Machine;
pub use command_history::CommandHistory;
pub use display::Display;
pub use execution_status::ExecutionStatus;
pub use index::Index;
pub use memory::Memory;
pub use registers::Registers;
pub use timers::Timers;

use crate::State;

mod command_history;
mod timers;
mod execution_status;
mod registers;
mod index;
mod memory;
mod display;

pub trait WindowContent {
    fn name(&self) -> &'static str;

    // todo: this should return a response
    fn ui(&mut self, ui: &mut Ui, machine: &Machine, state: &mut State);
}

pub struct Window {
    pub open: bool,
    content: Box<dyn WindowContent>,
}

impl Window {
    pub fn new(content: Box<dyn WindowContent>) -> Self {
        Self {
            open: true,
            content,
        }
    }

    pub fn name(&self) -> &'static str { self.content.name() }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine, state: &mut State) {
        egui::Window::new(self.content.name())
            .resizable(false)
            .open(&mut self.open)
            .show(ctx, |ui| { self.content.ui(ui, machine, state); });
    }
}