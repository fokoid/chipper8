use egui::Ui;

use chipper8::machine::Machine;
pub use display::Display;
pub use execution_status::ExecutionStatus;
pub use index::Index;
pub use memory::Memory;
pub use registers::Registers;
pub use timers::Timers;

pub mod repl;
mod timers;
mod execution_status;
mod registers;
mod index;
mod memory;
mod display;

pub trait Windowed {
    fn name(&self) -> &'static str;

    // todo: this should return a response
    fn ui(&mut self, ui: &mut Ui, machine: &Machine);
}