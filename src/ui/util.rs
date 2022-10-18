use std::fmt::{Debug, Formatter};

use egui::{Color32, Label, Response, Ui, Widget};
use egui::widget_text::RichText;

pub use formatting::{Address, Byte, Decimal, Register, Word};
pub use memory_display::MemoryDisplay;
pub use table::TabularData;

mod image_builder;
mod memory_display;
pub mod table;
mod formatting;

pub struct MonoLabel {
    text: String,
    background_color: Option<Color32>,
}

impl MonoLabel {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), background_color: None }
    }
}

impl Debug for MonoLabel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Widget for MonoLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut text = RichText::new(self.text).monospace().size(16.0);
        if let Some(background_color) = self.background_color {
            text = text.background_color(background_color);
        };
        ui.add(Label::new(text))
    }
}