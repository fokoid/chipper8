use egui::{Response, TextEdit, Ui};

pub use formatting::{Address, Byte, Decimal, Nibble, Register, Word};
pub use memory_display::MemoryDisplay;
pub use table::TabularData;

use crate::State;

mod image_builder;
mod memory_display;
pub mod table;
mod formatting;

/// helper function to wrap textbox add with global key capture management
pub fn add_text_edit(ui: &mut Ui, state: &mut State, widget: TextEdit) -> Response {
    let response = ui.add(widget);
    if response.has_focus() {
        state.key_capture_suspended = true;
    } else if response.lost_focus() {
        state.key_capture_suspended = false;
    };
    response
}