use std::collections::HashMap;

use egui::{Button, Color32, ComboBox, Key, Response, RichText, Ui};

use chipper8::machine::Machine;

use crate::State;
use crate::ui::util::Nibble;

use super::WindowContent;

// key layout of the original COSMAC VIP as well as most contemporary emulators
const KEYS: [u8; 16] = [
    0x1, 0x2, 0x3, 0xC,
    0x4, 0x5, 0x6, 0xD,
    0x7, 0x8, 0x9, 0xE,
    0xA, 0x0, 0xB, 0xF,
];

struct KeyBindings {
    pub active: String,
    bindings: HashMap<String, [Key; 16]>,
}

impl KeyBindings {
    fn new() -> Self {
        Self {
            active: String::from("Default"),
            bindings: HashMap::from([
                (String::from("Default"), [
                    Key::Num1,
                    Key::Num2,
                    Key::Num3,
                    Key::Num4,
                    Key::Q,
                    Key::W,
                    Key::E,
                    Key::R,
                    Key::A,
                    Key::S,
                    Key::D,
                    Key::F,
                    Key::Z,
                    Key::X,
                    Key::C,
                    Key::V,
                ], ),
                // because Moonlander Z key is dual purpose as CTRL by default
                (String::from("Moonlander"), [
                    Key::Num2,
                    Key::Num3,
                    Key::Num4,
                    Key::Num5,
                    Key::W,
                    Key::E,
                    Key::R,
                    Key::T,
                    Key::S,
                    Key::D,
                    Key::F,
                    Key::G,
                    Key::X,
                    Key::C,
                    Key::V,
                    Key::B,
                ], ),
            ])
        }
    }

    fn bindings(&self) -> Vec<&String> {
        self.bindings.keys().collect()
    }

    fn active_binding(&self) -> &[Key; 16] {
        &self.bindings[&self.active]
    }
}

pub struct Keypad {
    capture_keys: bool,
    show_binds: bool,
    bindings: KeyBindings,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            capture_keys: false,
            show_binds: false,
            bindings: KeyBindings::new(),
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
                        let key = self.bindings.active_binding()[index];
                        let bind = format!("{:?}", key).chars().last().unwrap();
                        format!("{} <{}>", Nibble::from(value), bind)
                    } else {
                        format!(" {} ", Nibble::from(value))
                    };
                    key_ui(ui, label, &mut state.keys[value as usize]);
                }
            });
        }
        if self.capture_keys {
            for (value, key) in KEYS.iter().zip(self.bindings.active_binding().iter()) {
                state.keys[*value as usize] |= ui.input().keys_down.contains(key);
            }
        }
        ui.separator();
        ui.checkbox(&mut self.capture_keys, "Capture key presses");
        ui.checkbox(&mut self.show_binds, "Show key bindings");
        ui.separator();
        ui.label("Select Keymap");
        ComboBox::from_label("")
            .selected_text(format!("{}", self.bindings.active))
            .show_ui(ui, |ui| {
                for binding in self.bindings.bindings.keys() {
                    ui.selectable_value(&mut self.bindings.active, binding.clone(), binding);
                }
            });
    }

    fn on_show(&mut self, response: Response) {
        if response.has_focus() {
            eprintln!("keypad focused");
        }
    }
}

fn key_ui(ui: &mut Ui, label: String, active: &mut bool) -> Response {
    let response = ui.add(Button::new(RichText::new(label)
        .size(24.0)
        .monospace())
        .fill(if *active { Color32::LIGHT_RED } else { Color32::DARK_GRAY })
    );
    *active = response.is_pointer_button_down_on();
    response
}