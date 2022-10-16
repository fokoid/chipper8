use egui::{Key, TextEdit, Ui};

use super::WindowContent;

use chipper8::machine::Machine;
use chipper8::instructions::{Command, MachineState, MetaCommand};

use crate::State;

pub struct InstructionGui {
    instructions: Vec<InstructionWidget>,
}

impl InstructionGui {
    pub fn new() -> Self {
        Self {
            instructions: vec![
                InstructionWidget::new("Clear Screen", "cls", vec![]),
                InstructionWidget::new("Jump", "jmp", vec!["Address"]),
                InstructionWidget::new("Set Index", "index set", vec!["Address"]),
                InstructionWidget::new("Set Sound Timer", "timer sound", vec!["Value"]),
                InstructionWidget::new("Set Register", "set", vec!["Register", "Value"]),
                InstructionWidget::new("Add to Register", "add", vec!["Register", "Value"]),
                InstructionWidget::new("Index to Font", "font", vec!["Register"]),
                InstructionWidget::new("Draw", "draw", vec!["Register X", "Register Y", "Height"]),
            ]
        }
    }
}

impl WindowContent for InstructionGui {
    fn name(&self) -> &'static str { "Instructions" }

    fn ui(&mut self, ui: &mut Ui, _machine: &Machine, state: &mut State) {
        for instruction in &mut self.instructions {
            instruction.ui(ui, state);
        }
    }
}

struct Arg {
    value: String,
    hint: &'static str,
}

impl Arg {
    fn new(hint: &'static str) -> Self {
        Self {
            value: String::new(),
            hint,
        }
    }
}

// todo: make this an actual egui widget
struct InstructionWidget {
    label: &'static str,
    instruction: &'static str,
    args: Vec<Arg>,
}

impl InstructionWidget {
    fn new(label: &'static str, instruction: &'static str, hints: Vec<&'static str>) -> Self {
        Self {
            label,
            instruction,
            args: hints.into_iter().map(Arg::new).collect(),
        }
    }

    // todo: confusing areas around the 'null' values inserted below
    // basically we submit `draw 1 <missing> 5` with the GUI it will get parsed as `draw 1 5` so the
    // error will say the third value is missing not the second. to fix this we insert 'null' where
    // text box args are left empty but this results in an error about integer parsing rather than a
    // missing value
    fn ui(&mut self, ui: &mut Ui, state: &mut State) {
        ui.horizontal(|ui| {
            let mut submitted = ui.button(self.label).clicked();
            for arg in &mut self.args {
                if ui.add(TextEdit::singleline(&mut arg.value)
                    .hint_text(arg.hint)
                    .desired_width(60.0)).lost_focus() && ui.input().key_pressed(Key::Enter) {
                    submitted = true;
                }
            }
            if submitted {
                state.parse_command(&format!("{} {}", self.instruction, self.args.iter().map(|arg| {
                    if arg.value.len() == 0 { String::from("null") } else { String::clone(&arg.value) }
                }).collect::<Vec<_>>().join(" ")));
            }
        });
    }
}
