pub use args::{BinaryOpArgs, DrawArgs, JumpArgs};
pub use instruction::{Flow, Graphics, Index, Input, Instruction, Memory};
pub use op_code::OpCode;

pub mod args;
mod op_code;
mod instruction;

#[cfg(test)]
mod tests;