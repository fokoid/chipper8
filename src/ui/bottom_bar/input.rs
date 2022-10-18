use std::cmp;

use egui::{Key, Response, TextEdit, TextStyle, Ui};
use ringbuffer::RingBuffer;

use crate::State;

pub struct Input {
    input: String,
    swap_input: String,
    history_index: isize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            swap_input: String::new(),
            history_index: 0,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, state: &mut State) -> Response {
        let response = ui.add(TextEdit::singleline(&mut self.input)
            .font(TextStyle::Monospace)
            .frame(false)
            .hint_text(">>>")
            .desired_width(250.0));
        // <RETURN> submits entered command
        if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
            state.parse_command(self.input.as_str());
            response.request_focus();
            self.reset();
        } else if response.has_focus() {
            // ^C clears textbox and resets history reverse search
            if ui.input().modifiers.ctrl && ui.input().key_pressed(Key::C) {
                self.reset();
            }
            // Up/Down arrows cycle through command history
            self.update_history_index(
                if ui.input().key_pressed(Key::ArrowUp) {
                    1
                } else if ui.input().key_pressed(Key::ArrowDown) {
                    -1
                } else {
                    0
                },
                state,
            );
        };
        ui.label(format!("{}: {}", self.history_index, self.swap_input));

        response
    }

    fn reset(&mut self) {
        self.input.clear();
        self.history_index = 0;
        self.swap_input.clear();
    }

    fn update_history_index(&mut self, step: isize, state: &State) {
        let history_index = -cmp::min(cmp::max(0, -self.history_index + step),
                                      state.command_history.items.len() as isize);
        if history_index != self.history_index {
            if self.history_index == 0 {
                self.swap_input = self.input.clone();
            }
            self.input = if history_index == 0 {
                self.swap_input.clone()
            } else {
                format!("{}", state.command_history.items[history_index].command)
            };
            self.history_index = history_index;
        }
    }
}
