use std::io;
use std::num::ParseIntError;

use thiserror::Error;

use crate::machine::{Instruction, OpCode};

#[derive(Error, Debug)]
pub enum Error {
    #[error("error parsing integer: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("integer too large for type: {0} {1}")]
    IntSizeError(String, u32),
    #[error("Syntax error in meta command: {0}")]
    MetaSyntaxError(String),
    #[error("syntax error: {0}")]
    SyntaxError(String),
    #[error("syntax error in opcode: {0}")]
    OpCodeSyntaxError(String),
    #[error("assembler error: no opcode for `{0}`")]
    NoOpcodeError(Instruction),
    #[error("invalid opcode: {0}")]
    InvalidOpCode(OpCode),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    // todo: move this into a separate error enum inside the machine module
    #[error("normal machine exit")]
    MachineExit,
}

pub type Result<T> = std::result::Result<T, Error>;