use egui::{Color32, Context, Frame, Response, Stroke, Ui};
use egui::style::Margin;
use egui_extras::{Size, TableBuilder};

use chipper8::machine::Machine;

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
            MonoLabel::new("Name", ),
            MonoLabel::new("Hex", ),
            MonoLabel::new("Decimal", ),
        ])
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        self.machine.registers.iter().enumerate().map(|(index, value)| {
            vec![
                MonoLabel::new(format!("V{:1X}", index), ),
                MonoLabel::new(format!("{:02X}", value), ),
                MonoLabel::new(format!("{:03}", value), ),
            ]
        }).collect()
    }
}

// todo: should we return a response?
pub fn stack_ui(ui: &mut Ui, machine: &Machine) {
    table::build(
        TableBuilder::new(ui)
            .striped(true)
            .stick_to_bottom(true)
            .resizable(false)
            .scroll(true),
        vec![50.0, 80.0, 50.0],
        StackHelper::new(machine),
    )
}

struct StackHelper<'a> {
    machine: &'a Machine,
}

impl<'a> StackHelper<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self { machine }
    }
}

impl<'a> TabularData for StackHelper<'a> {
    fn header(&self) -> Option<Vec<MonoLabel>> {
        Some(vec![
            MonoLabel::new("Depth", ),
            MonoLabel::new("Address", ),
            MonoLabel::new("Value", ),
        ])
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        self.machine.stack.data.iter().enumerate().map(|(index, address)| {
            let label = MonoLabel::new(format!("{:04X}", address));
            vec![
                MonoLabel::new(format!("{}", index + 1)),
                label.highlight_if(|| index == self.machine.stack.pointer),
                MonoLabel::new(format!("{:04X}", self.machine.word_at_address(*address as usize))),
            ]
        }).collect()
    }
}

struct PointersHelper<'a> {
    machine: &'a Machine,
}

impl<'a> PointersHelper<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self { machine }
    }
}

impl<'a> TabularData for PointersHelper<'a> {
    fn header(&self) -> Option<Vec<MonoLabel>> {
        None
    }

    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        // if let Ok(instruction) = machine.next_instruction() {
        //     ui.add(MonoLabel::new(format!("{}", instruction)));
        // };
        vec![
            vec![
                MonoLabel::new("PC", ),
                MonoLabel::new(format!("{:04X}", self.machine.program_counter), ),
                MonoLabel::new(format!("{:04X}", self.machine.at_program_counter()), ),
            ],
        ]
    }
}

pub fn pointers_ui(ui: &mut Ui, machine: &Machine) {
    table::build(
        TableBuilder::new(ui)
            .resizable(false)
            .scroll(false),
        vec![40.0, 40.0, 40.0],
        PointersHelper::new(machine),
    )
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
                MonoLabel::new("DELAY", ),
                MonoLabel::new(format!("{:02X}", self.machine.delay_timer), ),
                MonoLabel::new("", ),
            ],
            vec![
                MonoLabel::new("SOUND", )
                    .highlight_if(|| sound),
                MonoLabel::new(format!("{:02X}", self.machine.sound_timer), )
                    .highlight_if(|| sound),
                MonoLabel::new(if sound { "🔊" } else { "" }, ),
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