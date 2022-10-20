use egui::WidgetText;
use std::fmt::{Display, Formatter};

pub struct Byte(pub u8);

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Byte> for String {
    fn from(byte: Byte) -> Self {
        format!("{}", byte)
    }
}

impl From<Byte> for WidgetText {
    fn from(byte: Byte) -> Self {
        String::from(byte).into()
    }
}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}", self.0)
    }
}