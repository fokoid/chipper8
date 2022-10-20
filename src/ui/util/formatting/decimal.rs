use egui::WidgetText;
use std::fmt::{Display, Formatter};

use crate::machine::Pointer;

pub struct Decimal(pub u16);

impl From<u8> for Decimal {
    fn from(value: u8) -> Self {
        Self(value as u16)
    }
}

impl From<Pointer> for Decimal {
    fn from(pointer: Pointer) -> Self {
        Self(pointer as u16)
    }
}

impl From<Decimal> for String {
    fn from(decimal: Decimal) -> Self {
        format!("{}", decimal)
    }
}

impl From<Decimal> for WidgetText {
    fn from(decimal: Decimal) -> Self {
        String::from(decimal).into()
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>4}", self.0)
    }
}