use egui::Ui;
use egui_extras::TableBuilder;

use chipper8::machine::Machine;

use crate::State;
use crate::ui::util::MonoLabel;
use crate::ui::util::table::{ColumnSpec, TableSpec, TabularData};

use super::WindowContent;

pub struct Timers {
    table_spec: TableSpec,
}

impl Timers {
    pub fn new() -> Self {
        Self {
            table_spec: TableSpec {
                show_header: false,
                enable_context_menu: false,
                columns: vec![
                    ColumnSpec::fixed("Label", 50.0),
                    ColumnSpec::fixed("Value", 20.0),
                    ColumnSpec::fixed("Icon", 20.0),
                ],
            }
        }
    }
}

impl WindowContent for Timers {
    fn name(&self) -> &'static str {
        "Timers"
    }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, _state: &mut State) {
        self.table_spec.build(
            TableBuilder::new(ui)
                .resizable(false)
                .scroll(false),
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