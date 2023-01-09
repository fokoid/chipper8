use std::fmt::{Debug, Display, Formatter};

use crate::machine::types::Word;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpCode(pub Word);

impl OpCode {
    pub fn bytes(&self) -> [u8; 2] {
        self.0.0.to_be_bytes()
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for OpCode {
    fn into(self) -> String {
        format!("{}", self)
    }
}
