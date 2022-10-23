use std::fmt::{Display, Formatter};

use ux::u4;

use crate::Error;
use crate::command::tokens::Token;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Register {
    pub index: u4,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "V{:01X}", self.index)
    }
}

impl TryFrom<u8> for Register {
    type Error = Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            index: u4::try_from(value).map_err(|_error| {
                Error::IntSizeError(String::from("register"), value.into())
            })?
        })
    }
}

impl TryFrom<Token<'_>> for Register {
    type Error = Error;

    fn try_from(token: Token) -> std::result::Result<Self, Self::Error> {
        match token {
            Token::Register(s) => {
                let value = u8::from_str_radix(&s[1..], 16)?;
                Ok(value.try_into()?)
            }
            x => Err(Error::SyntaxError(format!("expected register, found {:?}", x))),
        }
    }
}