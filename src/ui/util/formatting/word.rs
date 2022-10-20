use egui::WidgetText;
use std::fmt::{Display, Formatter};

pub struct Word(pub u16);

impl From<u16> for Word {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Word> for String {
    fn from(word: Word) -> Self {
        format!("{}", word)
    }
}

impl From<Word> for WidgetText {
    fn from(word: Word) -> Self {
        String::from(word).into()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06X}", self.0)
    }
}