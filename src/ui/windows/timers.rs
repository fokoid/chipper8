use egui::Ui;
use egui_extras::TableBuilder;

use chipper8::machine::Machine;

use crate::ui::util::{table, TabularData};
use crate::ui::util::MonoLabel;

use super::WindowContent;

pub struct Timers {}

impl Timers {
    pub fn new() -> Self { Self {} }
}

impl WindowContent for Timers {
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
        vec![
            timer_row("Delay", self.machine.delay_timer, None),
            timer_row("Sound", self.machine.sound_timer, Some('🔊')),
        ]
    }
}

fn timer_row(label: &str, timer: u8, active_icon: Option<char>) -> Vec<MonoLabel> {
    let active_icon = if timer > 0 { active_icon } else { None };
    vec![
        MonoLabel::new(label),
        MonoLabel::new(format!("{:02X}", timer)),
        MonoLabel::new(active_icon.unwrap_or(' ')),
    ]
}