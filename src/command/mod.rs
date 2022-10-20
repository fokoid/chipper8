use std::fmt::{Debug, Display, Formatter};

pub use machine_state::MachineState;
pub use meta_command::MetaCommand;
use tokens::{Token, Tokens};

use crate::machine::{Instruction, OpCode};
use crate::Result;

pub mod tokens;
pub mod meta_command;
mod machine_state;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Meta(MetaCommand),
    Instruction(Instruction),
}

impl Command {
    pub fn parse(mut tokens: Tokens) -> Result<Option<Self>> {
        match tokens.peek() {
            None | Some(Token::None) => Ok(None),
            Some(Token::Hex(_)) => {
                let opcode = &OpCode::parse(tokens)?;
                Ok(Some(Self::Instruction(opcode.as_instruction()?)))
            }
            Some(Token::Meta(_)) => Ok(Some(Self::Meta(MetaCommand::parse(tokens)?))),
            Some(Token::Other(_)) => Ok(Some(Self::Instruction(Instruction::parse(tokens)?))),
        }
    }
}

impl Command {
    pub fn opcode(&self) -> Option<OpCode> {
        match self {
            Self::Meta(_) => None,
            Self::Instruction(instruction) => Some(instruction.into()),
        }
    }

    pub fn is_meta(&self) -> bool {
        match self {
            Self::Meta(_) => true,
            Self::Instruction(_) => false,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Meta(meta) => write!(f, "{}", meta),
            Self::Instruction(instruction) => write!(f, "{}", instruction),
        }
    }
}
