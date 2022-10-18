use chipper8::machine::Machine;
use egui::{Button, Color32, Response, RichText, Ui};

use crate::State;
use crate::ui::util::Nibble;

use super::WindowContent;

pub struct Keypad {}

impl Keypad {
    pub fn new() -> Self {
        Self {}
    }
}

impl WindowContent for Keypad {
    fn name(&self) -> &'static str {
        "Keypad"
    }

    fn ui(&mut self, ui: &mut Ui, _machine: &Machine, state: &mut State) {
        let mut key_buffer = None;
        for row in 0..4 {
            ui.horizontal(|ui| {
                for col in 0..4 {
                    key_ui(ui, row * 4 + col, state.is_key_down(row * 4 + col), &mut key_buffer);
                }
            });
        }
        state.key_pressed = key_buffer.take();
    }
}

fn key_ui(ui: &mut Ui, value: u8, active: bool, key_buffer: &mut Option<u8>) -> Response {
    let response = ui.add(Button::new(RichText::new(Nibble::from(value))
        .size(24.0)
        .monospace())
        .fill(if active { Color32::LIGHT_RED } else { Color32::DARK_GRAY })
    );
    if response.is_pointer_button_down_on() {
        *key_buffer = Some(value);
    };
    response
}