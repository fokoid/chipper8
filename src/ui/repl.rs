use egui::{Align, Color32, Context, Frame, Layout, Response, Stroke, TextStyle, Ui};
use egui::style::Margin;
use egui::widgets::TextEdit;
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};

use chipper8::instructions::Command;

use crate::ui::util::MonoLabel;
use crate::ui::table::{self, TabularData};

// hard coded based on current (also hard coded) UI element sizes
const REPL_HISTORY_SIZE: usize = 16;

struct HistoryItem {
    command: Command,
    user: bool,
    count: usize,
}

impl TabularData for &AllocRingBuffer<HistoryItem> {
    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        self.iter().map(|item| {
            vec![
                MonoLabel::new(
                    if item.command.is_meta() {
                        "M"
                    } else if item.user {
                        "U"
                    } else {
                        " "
                    }
                ),
                MonoLabel::new(match item.command.opcode() {
                    None => String::from(""),
                    Some(opcode) => format!("{}", opcode),
                }),
                MonoLabel::new(format!("{}", item.command)),
                MonoLabel::new(
                    if item.count == 1 {
                        String::from("  ")
                    } else if item.count < 100
                    {
                        format!("{}", item.count)
                    } else { String::from("💯") }
                ),
            ]
        }).collect()
    }
}

pub struct Repl {
    input: String,
    history: AllocRingBuffer<HistoryItem>,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            history: AllocRingBuffer::with_capacity(REPL_HISTORY_SIZE),
            input: String::new(),
        }
    }

    pub fn add_history(&mut self, command: &Command, user: bool) {
        match self.history.back_mut() {
            Some(item) if item.command == *command && item.user == user => item.count += 1,
            _ => self.history.push(HistoryItem { command: command.clone(), user, count: 1 }),
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, command_buffer: &mut Option<Command>) -> Response {
        ui.with_layout(
            Layout::bottom_up(Align::LEFT),
            |ui| {
                let submitted = repl_input_ui(ui, &mut self.input).lost_focus();
                repl_history_ui(ui, &self.history);
                if submitted {
                    match Command::parse(self.input.as_str().into()) {
                        Ok(Some(command)) => {
                            self.input.clear();
                            self.add_history(&command, true);
                            command_buffer.replace(command);
                        }
                        Ok(None) => {}
                        Err(error) => {
                            eprintln!("{:?}", error);
                        }
                    };
                };
            }
        ).response
    }
}

fn repl_input_ui(ui: &mut Ui, text: &mut String) -> Response {
    ui.add(TextEdit::singleline(text)
        .font(TextStyle::Monospace)
        .frame(false)
        .hint_text(">>>")
        .desired_width(250.0))
}

fn repl_history_ui(ui: &mut Ui, items: &AllocRingBuffer<HistoryItem>) -> () {
    table::build(
        ui,
        vec![10.0, 40.0, 160.0, 20.0],
        items,
    )
}