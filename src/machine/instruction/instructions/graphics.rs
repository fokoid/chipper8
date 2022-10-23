use std::fmt::Debug;

use ux::u4;

use super::Register;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrawArgs {
    pub x: Register,
    pub y: Register,
    pub height: u4,
}