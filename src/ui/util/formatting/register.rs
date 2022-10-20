use egui::WidgetText;
use std::fmt::{Display, Formatter};

pub struct Register(pub u8);

impl From<u8> for Register {
    fn from(register: u8) -> Self {
        Self(register & 0xF as u8)
    }
}

impl From<usize> for Register {
    fn from(register: usize) -> Self {
        (register as u8).into()
    }
}

impl From<Register> for String {
    fn from(register: Register) -> Self {
        format!("{}", register)
    }
}

impl From<Register> for WidgetText {
    fn from(register: Register) -> Self {
        String::from(register).into()
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "V{:01X}", self.0)
    }
}