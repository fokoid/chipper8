use std::fmt::{Display, Formatter};

use egui::WidgetText;

pub struct Nibble(pub u8);

impl From<u8> for Nibble {
    fn from(register: u8) -> Self {
        Self(register & 0xF)
    }
}

impl From<usize> for Nibble {
    fn from(register: usize) -> Self {
        Self((register & 0xF) as u8)
    }
}

impl From<Nibble> for String {
    fn from(nibble: Nibble) -> Self {
        format!("{}", nibble)
    }
}

impl From<Nibble> for WidgetText {
    fn from(nibble: Nibble) -> Self {
        String::from(nibble).into()
    }
}

impl Display for Nibble {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:01X}", self.0)
    }
}