use std::collections::HashMap;

use egui::{Key, Ui};

// key layout of the original COSMAC VIP as well as most contemporary emulators
pub const KEYS: [u8; 16] = [
    0x1, 0x2, 0x3, 0xC,
    0x4, 0x5, 0x6, 0xD,
    0x7, 0x8, 0x9, 0xE,
    0xA, 0x0, 0xB, 0xF,
];

pub struct KeyBindings {
    pub active: String,
    bindings: HashMap<String, [Key; 16]>,
}

impl KeyBindings {
    fn new() -> Self {
        Self {
            active: String::from("Moonlander"),
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
            ]),
        }
    }

    pub fn active_binding(&self) -> &[Key; 16] {
        &self.bindings[&self.active]
    }

    pub fn available_bindings(&self) -> Vec<String> {
        let mut result = Vec::new();
        for key in self.bindings.keys() {
            result.push(key.clone());
        }
        result
    }
}

pub struct KeyCapture {
    pub enabled: bool,
    pub bindings: KeyBindings,
    pub keys: [bool; 16],
}

impl KeyCapture {
    pub fn new() -> Self {
        Self {
            enabled: true,
            bindings: KeyBindings::new(),
            keys: [false; 16],
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if !self.enabled { return; }

        for (value, key) in KEYS.iter().zip(self.bindings.active_binding().iter()) {
            self.keys[*value as usize] = ui.input().keys_down.contains(key);
        }
    }

    pub fn key(&self) -> Option<u8> {
        // returns which key is pressed if any
        // todo: decide what to do when multiple keys are pressed at once
        // (for now we take the first)
        for (value, pressed) in self.keys.iter().enumerate() {
            if *pressed {
                return Some((value & 0xF) as u8);
            }
        }
        None
    }
}