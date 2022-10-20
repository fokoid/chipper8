use std::fmt::{Display, Formatter};

use crate::{Error, Result};

use super::tokens::{Token, Tokens};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MachineState {
    Demo,
}

impl MachineState {
    pub fn parse(mut tokens: Tokens) -> Result<Option<MachineState>> {
        match tokens.next() {
            Some(Token::Other("demo")) => Ok(Some(MachineState::Demo)),
            None => Ok(None),
            Some(x) => Err(Error::MetaSyntaxError(format!("not a valid machine state identifier: {:?}", x))),
        }
    }
}

impl Display for MachineState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Demo => write!(f, "{}", "demo"),
        }
    }
}