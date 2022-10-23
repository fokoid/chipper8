pub use args::{DrawArgs, SetArgs};
pub use instruction::Instruction;
pub use op_code::OpCode;

pub mod args;
mod op_code;
mod instruction;

#[cfg(test)]
mod tests;