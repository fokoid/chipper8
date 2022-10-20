use egui::{Ui, TextStyle, WidgetText};

use chipper8::machine::Machine;

use crate::State;
use crate::ui::util::Byte;
use crate::ui::util::table::{ColumnSpec, TableSpec, TabularData};

use super::WindowContent;

pub struct Timers {
    table_spec: TableSpec,
}

impl Timers {
    pub fn new() -> Self {
        Self {
            table_spec: TableSpec::new(
                vec![
                    ColumnSpec::fixed("Label", 50.0),
                    ColumnSpec::fixed("Value", 30.0),
                    ColumnSpec::fixed("Icon", 20.0),
                ],
            ).header(false).context_menu(false)
        }
    }
}

impl WindowContent for Timers {
    fn name(&self) -> &'static str {
        "Timers"
    }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, _state: &mut State) {
        ui.style_mut().override_text_style = Some(TextStyle::Monospace);
        self.table_spec.draw(ui, TimersHelper::new(machine))
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
    fn rows(&self) -> Vec<Vec<WidgetText>> {
        vec![
            timer_row("Delay", self.machine.delay_timer, None),
            timer_row("Sound", self.machine.sound_timer, Some('🔊')),
        ]
    }
}

fn timer_row(label: &str, timer: u8, active_icon: Option<char>) -> Vec<WidgetText> {
    let active_icon = if timer > 0 { active_icon } else { None };
    vec![
        label.into(),
        Byte::from(timer).into(),
        String::from(active_icon.unwrap_or(' ')).into(),
    ]
}