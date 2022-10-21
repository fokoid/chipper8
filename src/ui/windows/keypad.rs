use egui::{Button, Color32, ComboBox, Response, RichText, Ui};

use crate::machine::Machine;
use crate::ui::State;
use crate::ui::util::key_capture::KEYS;
use crate::ui::util::Nibble;

use super::WindowContent;

pub struct Keypad {
    show_binds: bool,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            show_binds: false,
        }
    }
}

impl WindowContent for Keypad {
    fn name(&self) -> &'static str {
        "Keypad"
    }

    fn ui(&mut self, ui: &mut Ui, _machine: &Machine, state: &mut State) {
        for row in 0..4 {
            ui.horizontal(|ui| {
                for col in 0..4 {
                    let index = row * 4 + col;
                    let value = KEYS[index];
                    let label = if self.show_binds {
                        let key = state.key_capture.bindings.active_binding()[index];
                        let bind = format!("{:?}", key).chars().last().unwrap();
                        format!("{} <{}>", Nibble::from(value), bind)
                    } else {
                        format!(" {} ", Nibble::from(value))
                    };
                    key_ui(ui, label, &mut state.key_capture.keys[value as usize]);
                }
            });
        }
        ui.separator();
        ui.checkbox(&mut state.key_capture.enabled, "Capture key presses");
        ui.checkbox(&mut self.show_binds, "Show key bindings");
        ui.separator();
        ui.label("Select Keymap");
        ComboBox::from_label("")
            .selected_text(format!("{}", state.key_capture.bindings.active))
            .show_ui(ui, |ui| {
                for binding in state.key_capture.bindings.available_bindings() {
                    ui.selectable_value(&mut state.key_capture.bindings.active, binding.clone(), binding);
                }
            });
    }
}

fn key_ui(ui: &mut Ui, label: String, active: &mut bool) -> Response {
    let response = ui.add(Button::new(RichText::new(label)
        .size(24.0)
        .monospace())
        .fill(if *active { Color32::LIGHT_RED } else { Color32::DARK_GRAY })
    );
    *active |= response.is_pointer_button_down_on();
    response
}