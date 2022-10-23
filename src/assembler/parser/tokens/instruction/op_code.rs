use crate::{Error, Result};
use crate::machine::OpCode;

use super::{Token, Tokens};

impl TryFrom<Tokens<'_>> for OpCode {
    type Error = Error;

    fn try_from(mut tokens: Tokens) -> Result<Self> {
        match tokens.next() {
            Some(Token::Hex(s)) => {
                Ok(Self(u16::from_str_radix(&s[2..], 16)?))
            }
            x => Err(Error::OpCodeSyntaxError(format!("{:?}", x))),
        }
    }
}
