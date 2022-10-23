use std::fmt::{Display, Formatter};

use ux::u4;

use crate::Error;

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
        let index: u4 = value.try_into().map_err(|_error|
            Error::IntSizeError(String::from("nibble"), value.into())
        )?;
        Ok(Self { index } )
    }
}