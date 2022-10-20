use std::io;
use std::num::ParseIntError;

use thiserror::Error;

use crate::machine::OpCode;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error parsing integer: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Syntax error in meta command: {0}")]
    MetaSyntaxError(String),
    #[error("syntax error: {0}")]
    SyntaxError(String),
    #[error("syntax error in opcode: {0}")]
    OpCodeSyntaxError(String),
    #[error("invalid opcode: {0}")]
    InvalidOpCode(OpCode),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;