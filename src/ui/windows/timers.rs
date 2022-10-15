use egui::Ui;
use egui_extras::TableBuilder;

use chipper8::machine::Machine;

use crate::ui::util::{table, TabularData};
use crate::ui::util::MonoLabel;

use super::Windowed;

pub struct Timers {}

impl Timers {
    pub fn new() -> Self { Self {} }
}

impl Windowed for Timers {
    fn name(&self) -> &'static str {
        "Timers"
    }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine) {
        table::build(
            TableBuilder::new(ui)
                .resizable(false)
                .scroll(false),
            vec![50.0, 20.0, 20.0],
            TimersHelper::new(machine),
        )
    }
}

struct TimersHelper<'a> {
    machine: &'a Machine,
}

impl<'a> TimersHelper<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self { machine }
    }
}

impl<'a> TabularData for TimersHelper<'a> {
    fn header(&self) -> Option<Vec<MonoLabel>> {
        None
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        let sound = self.machine.sound_timer > 0;
        vec![
            vec![
                MonoLabel::new("DELAY"),
                MonoLabel::new(format!("{:02X}", self.machine.delay_timer)),
                MonoLabel::new(""),
            ],
            vec![
                MonoLabel::new("SOUND"),
                MonoLabel::new(format!("{:02X}", self.machine.sound_timer)),
                MonoLabel::new(if sound { "🔊" } else { "" }),
            ],
        ]
    }
}