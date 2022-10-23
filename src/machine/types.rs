use std::fmt::{Display, Formatter};

use ux::{u12, u4};

use crate::{Error, Result};

pub type Timer = u8;
pub type Pointer = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Register(pub u4);

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "V{:01X}", self.0)
    }
}

impl TryFrom<u8> for Register {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        let index: u4 = value.try_into().map_err(|_error|
            Error::IntSizeError(String::from("nibble"), value.into())
        )?;
        Ok(Self(index))
    }
}

impl From<&Register> for u8 {
    fn from(register: &Register) -> Self {
        u8::from(register.0)
    }
}

impl From<&Register> for usize {
    fn from(register: &Register) -> Self {
        u8::from(register) as usize
    }
}