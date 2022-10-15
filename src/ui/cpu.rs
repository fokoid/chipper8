use egui::{Align, Color32, Context, Frame, Layout, Response, Stroke, Ui};
use egui::style::Margin;
use egui_extras::{Size, TableBuilder};

use chipper8::machine::Machine;

use crate::ui::{program_counter, stack};
use crate::ui::table::{self, TabularData};
use crate::ui::util::MonoLabel;

// todo: should we return a response?
pub fn registers_ui(ui: &mut Ui, machine: &Machine) {
    table::build(
        TableBuilder::new(ui)
            .striped(true)
            .stick_to_bottom(true)
            .resizable(false)
            .scroll(false)
        // .column(Size::relative(0.5))
        // .column(Size::relative(0.5))
        ,
        vec![60.0, 60.0, 60.0],
        RegistersHelper::new(machine),
    )
}

struct RegistersHelper<'a> {
    machine: &'a Machine,
}

impl<'a> RegistersHelper<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self { machine }
    }
}

impl<'a> TabularData for RegistersHelper<'a> {
    fn header(&self) -> Option<Vec<MonoLabel>> {
        Some(vec![
            MonoLabel::new("Name"),
            MonoLabel::new("Hex"),
            MonoLabel::new("Decimal"),
        ])
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        self.machine.registers.iter().enumerate().map(|(index, value)| {
            vec![
                MonoLabel::new(format!("V{:1X}", index)),
                MonoLabel::new(format!("{:02X}", value)),
                MonoLabel::new(format!("{:03}", value)),
            ]
        }).collect()
    }
}

// todo: should we return a response?
pub fn program_status_ui(ui: &mut Ui, machine: &Machine) {
    ui.push_id(0, |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add(MonoLabel::new("Program Counter"))
        });
        program_counter::program_counter_ui(ui, machine);
    });
    ui.push_id(1, |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add(MonoLabel::new("Stack"))
        });
        stack::stack_ui(ui, machine);
    });
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