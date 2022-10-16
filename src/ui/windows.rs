use egui::{Context, Ui};

use chipper8::machine::Machine;
use command_history::CommandHistory;
use display::Display;
use execution_status::ExecutionStatus;
use index::Index;
use memory::Memory;
use command_gui::CommandGui;
use registers::Registers;
use timers::Timers;
pub use execution_status::ProgramCounterHelper;

use crate::State;

mod command_history;
mod timers;
mod execution_status;
mod registers;
mod index;
mod memory;
mod display;
mod command_gui;

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
            .resizable(true)
            .open(&mut self.open)
            .show(ctx, |ui| { self.content.ui(ui, machine, state); });
    }
}

pub fn create_all() -> Vec<Window> {
    vec![
        Window::new(Box::new(CommandHistory::new())),
        Window::new(Box::new(Display::new())),
        Window::new(Box::new(Memory::new())),
        Window::new(Box::new(Registers::new())),
        Window::new(Box::new(Index::new())),
        Window::new(Box::new(Timers::new())),
        Window::new(Box::new(ExecutionStatus::new())),
        Window::new(Box::new(CommandGui::meta())),
        Window::new(Box::new(CommandGui::instruction())),
    ]
}