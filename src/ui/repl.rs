use egui::{Color32, Context, Frame, Stroke, TextStyle, Ui};
use egui::widgets::TextEdit;
use egui::style::Margin;
use egui_extras::{Size, TableBuilder, TableRow};
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};

use chipper8::instructions::{self, Command};
use crate::ui::util;

// hard coded based on current (also hard coded) UI element sizes
const REPL_HISTORY_SIZE: usize = 16;

struct HistoryItem {
    command: Command,
    user: bool,
    count: usize,
}

impl HistoryItem {
    fn draw_row(&self, row: &mut TableRow) {
        row.col(|ui| {
            ui.label(util::monospace(
                if self.command.is_meta() {
                    "M"
                } else if self.user {
                    "U"
                } else {
                    " "
                }
            ));
        });
        row.col(|ui| {
            ui.label(util::monospace(&match self.command.opcode() {
                None => String::from(""),
                Some(opcode) => format!("{}", opcode),
            }));
        });
        row.col(|ui| {
            ui.label(util::monospace(&format!("{}", self.command)));
        });
        row.col(|ui| {
            ui.label(util::monospace(
                &if self.count == 1 {
                    String::from("  ")
                } else if self.count < 100
                {
                    format!("{}", self.count)
                } else { String::from("💯") }
            ));
        });
    }
}

pub struct Repl {
    input: Input,
    history: History,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            history: History::new(),
        }
    }

    // todo: get rid of this
    pub fn add_history(&mut self, command: &Command, user: bool) {
        self.history.add(command, user);
    }

    pub fn draw(&mut self, ctx: &Context) -> instructions::Result<Option<Command>> {
        egui::SidePanel::left("console")
            .resizable(false)
            .min_width(265.0)
            .max_width(265.0)
            .frame(Frame::default().stroke(Stroke::new(2.0, Color32::DARK_GRAY)))
            .show(ctx, |ui| {
                self.history.draw(ui);
                Ok(self.input.draw(ui)?.map(|command| {
                    self.history.add(&command, true);
                    command
                }))
            }).inner
    }
}

struct Input {
    user_input: String,
}

impl Input {
    fn new() -> Self {
        Self { user_input: String::new() }
    }

    fn draw(&mut self, ui: &mut Ui) -> instructions::Result<Option<Command>> {
        egui::TopBottomPanel::bottom("input")
            .resizable(false)
            .min_height(30.0)
            .max_height(30.0)
            .frame(Frame::default()
                .inner_margin(Margin::symmetric(5.0, 0.0))
                .fill(Color32::DARK_GRAY))
            .show_inside(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let input = ui.add(TextEdit::singleline(&mut self.user_input)
                        .font(TextStyle::Monospace)
                        .desired_width(250.0));
                    Ok(if input.lost_focus() {
                        Command::parse(self.user_input.as_str().into())?.map(|command| {
                            self.user_input.clear();
                            input.request_focus();
                            command
                        })
                    } else {
                        None
                    })
                }).inner
            }).inner
    }
}

struct History {
    items: AllocRingBuffer<HistoryItem>,
}

impl History {
    fn new() -> Self {
        Self {
            items: AllocRingBuffer::with_capacity(REPL_HISTORY_SIZE),
        }
    }

    pub fn add(&mut self, command: &Command, user: bool) {
        match self.items.back_mut() {
            Some(item) if item.command == *command && item.user == user => item.count += 1,
            _ => self.items.push(HistoryItem { command: command.clone(), user, count: 1 }),
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::top("history")
            .resizable(false)
            .min_height(335.0)
            .max_height(335.0)
            .frame(Frame::none().inner_margin(Margin::symmetric(5.0, 5.0)))
            .show_inside(ui, |ui| {
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .column(Size::exact(10.0))
                    .column(Size::exact(40.0))
                    .column(Size::exact(160.0))
                    .column(Size::exact(20.0))
                    .resizable(false)
                    .scroll(false)
                    .stick_to_bottom(true);
                table.body(|mut body| {
                    self.items.iter().for_each(|item| {
                        body.row(18.0, |mut row| item.draw_row(&mut row) );
                    })
                });
            });
    }
}