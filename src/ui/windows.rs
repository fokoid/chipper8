use egui::{Context, Response, Ui};

use chipper8::machine::Machine;
use command_gui::CommandGui;
use command_history::CommandHistory;
use display::Display;
use execution_status::ExecutionStatus;
pub use execution_status::ProgramCounterHelper;
use index::Index;
use keypad::Keypad;
use memory::Memory;
use registers::Registers;
use timers::Timers;

use crate::State;

mod command_history;
mod timers;
mod execution_status;
mod registers;
mod index;
mod memory;
mod display;
mod command_gui;
mod keypad;

pub trait WindowContent {
    fn name(&self) -> &'static str;

    // todo: this should return a response
    fn ui(&mut self, ui: &mut Ui, machine: &Machine, state: &mut State);

    fn on_show(&mut self, _response: Response) {}
}

pub struct Window {
    pub open: bool,
    pub content: Box<dyn WindowContent>,
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
        let response = egui::Window::new(self.content.name())
            .resizable(true)
            .open(&mut self.open)
            .show(ctx, |ui| { self.content.ui(ui, machine, state); });
        if let Some(response) = response {
            self.content.on_show(response.response);
        }
    }
}

pub fn create_all() -> Vec<Window> {
    vec![
        Window::new(Box::new(CommandHistory::new())),
        Window::new(Box::new(CommandGui::meta())),
        Window::new(Box::new(CommandGui::instruction())),
        Window::new(Box::new(Display::new())),
        Window::new(Box::new(Memory::new())),
        Window::new(Box::new(Index::new())),
        Window::new(Box::new(Timers::new())),
        Window::new(Box::new(Registers::new())),
        Window::new(Box::new(ExecutionStatus::new())),
        Window::new(Box::new(Keypad::new())),
    ]
}