pub use args::{AddressArgs, DrawArgs, SetArgs};
pub use instruction::{Flow, Graphics, Instruction};
pub use op_code::OpCode;

pub mod args;
mod op_code;
mod instruction;

#[cfg(test)]
mod tests;