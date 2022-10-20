use std::fmt::{Display, Formatter};

use egui::WidgetText;

use chipper8::machine::Pointer;

pub struct Address(pub u16);

impl From<Pointer> for Address {
    fn from(pointer: Pointer) -> Self {
        Self(pointer as u16)
    }
}

impl From<Address> for String {
    fn from(address: Address) -> Self {
        format!("{}", address)
    }
}

impl From<Address> for WidgetText {
    fn from(address: Address) -> Self {
        String::from(address).into()
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#05X}", self.0)
    }
}