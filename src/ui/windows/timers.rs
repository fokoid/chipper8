use egui::Ui;
use egui_extras::TableBuilder;

use chipper8::machine::Machine;

use crate::ui::util::{table, TabularData};
use crate::ui::util::MonoLabel;

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
                MonoLabel::new("SOUND")
                    .highlight_if(|| sound),
                MonoLabel::new(format!("{:02X}", self.machine.sound_timer))
                    .highlight_if(|| sound),
                MonoLabel::new(if sound { "🔊" } else { "" }),
            ],
        ]
    }
}

pub fn timers_ui(ui: &mut Ui, machine: &Machine) {
    table::build(
        TableBuilder::new(ui)
            .resizable(false)
            .scroll(false),
        vec![50.0, 20.0, 20.0],
        TimersHelper::new(machine),
    )
}