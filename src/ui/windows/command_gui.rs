use egui::{Key, TextEdit, Ui};

use crate::machine::Machine;
use crate::ui::State;
use crate::ui::util;

use super::WindowContent;

struct Arg {
    value: String,
    hint: &'static str,
}

// todo: different types of arg with type-dependent field behaviour
// for example:
//   - MachineState (for reset meta command): dropdown
//   - Register: only allow single digit of hex
// etc
impl Arg {
    fn new(hint: &'static str) -> Self {
        Self {
            value: String::new(),
            hint,
        }
    }
}

// todo: make this an actual egui widget
struct CommandWidget {
    label: &'static str,
    instruction: &'static str,
    args: Vec<Arg>,
}

impl CommandWidget {
    fn new(label: &'static str, instruction: &'static str, hints: Vec<&'static str>) -> Self {
        Self {
            label,
            instruction,
            args: hints.into_iter().map(Arg::new).collect(),
        }
    }

    // todo: confusing error messages around missing args
    // if we try to submit via the GUI `draw 1 <missing> 5` it will get parsed as `draw 1 5`, so the
    // syntax error will complain about the third value being missing not second
    fn ui(&mut self, ui: &mut Ui, state: &mut State) {
        ui.horizontal(|ui| {
            let mut submitted = ui.button(self.label).clicked();
            for arg in &mut self.args {
                if util::add_text_edit(ui, state, TextEdit::singleline(&mut arg.value)
                    .hint_text(arg.hint)
                    .desired_width(60.0)).lost_focus() && ui.input().key_pressed(Key::Enter) {
                    submitted = true;
                }
            }
            if submitted {
                state.parse_command(&format!("{} {}", self.instruction, self.args.iter().map(|arg| {
                    arg.value.clone()
                }).collect::<Vec<_>>().join(" ")));
            }
        });
    }
}

pub struct CommandGui {
    name: &'static str,
    commands: Vec<CommandWidget>,
}

impl CommandGui {
    pub fn instruction() -> Self {
        Self {
            name: "Instruction GUI",
            commands: vec![
                CommandWidget::new("Clear Screen", "graphics clear", vec![]),
                CommandWidget::new("Jump", "jump", vec!["Address"]),
                CommandWidget::new("Set Index", "index", vec!["Address"]),
                CommandWidget::new("Set Delay Timer", "set delay", vec!["Register"]),
                CommandWidget::new("Get Delay Timer", "get timer", vec!["Register"]),
                CommandWidget::new("Set Sound Timer", "set sound", vec!["Register"]),
                CommandWidget::new("Set Register", "set", vec!["Register", "Register or Value"]),
                CommandWidget::new("Add to Register", "add", vec!["Register", "Register or Value"]),
                CommandWidget::new("Index to Font", "font", vec!["Register"]),
                CommandWidget::new("Draw", "graphics draw", vec!["Register X", "Register Y", "Height"]),
                CommandWidget::new("Exit", "exit", vec![]),
            ],
        }
    }

    pub fn meta() -> Self {
        Self {
            name: "Meta Command GUI",
            // todo: split into horizontal groups
            commands: vec![
                CommandWidget::new("Play", ":play", vec![]),
                CommandWidget::new("Pause", ":pause", vec![]),
                CommandWidget::new("Play / Pause", ":play-pause", vec![]),
                CommandWidget::new("Tick", ":tick", vec![]),
                // todo: dropdown of allowed machine states
                CommandWidget::new("Reset", ":reset", vec!["State"]),
                // todo: dropdown of available ROMs
                CommandWidget::new("Load ROM", ":load", vec!["Filename", "Address"]),
                CommandWidget::new("Load IBM", ":load ibm", vec![]),
                CommandWidget::new("Load Test: BC", ":load tests/bc", vec![]),
                CommandWidget::new("Load Test: corax89", ":load tests/corax89", vec![]),
                CommandWidget::new("Unload ROM", ":unload", vec![]),
                CommandWidget::new("Dump Machine", ":dump", vec!["Filename"]),
                CommandWidget::new("Load Machine", ":load-machine", vec!["Filename"]),
            ],
        }
    }
}

impl WindowContent for CommandGui {
    fn name(&self) -> &'static str { self.name }

    fn ui(&mut self, ui: &mut Ui, _machine: &Machine, state: &mut State) {
        self.commands.iter_mut().for_each(|command| command.ui(ui, state));
    }
}