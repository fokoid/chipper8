use std::fmt::{Display, Formatter};

use egui::WidgetText;

use crate::machine;

pub struct Address(pub u16);

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

// todo: get rid of this
impl From<usize> for Address {
    fn from(value: usize) -> Self {
        let value = (value & 0x0000FFFF) as u16;
        value.into()
    }
}

impl From<&machine::Address> for Address {
    fn from(address: &machine::Address) -> Self {
        Self(u16::from(address))
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