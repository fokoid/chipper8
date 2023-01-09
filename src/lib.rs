pub use emulator::{Emulator, EmulatorConfig};
pub use errors::{Error, Result};
pub use machine::Machine;

pub mod machine;
pub mod command;
pub mod errors;
pub mod ui;
pub mod emulator;
mod parser;
