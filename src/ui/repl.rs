use egui::{Color32, Context, Frame, Stroke, TextStyle};
use egui::widgets::TextEdit;
use egui::style::Margin;
use egui_extras::{Size, TableBuilder};
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

pub struct Repl {
    user_input: String,
    history: AllocRingBuffer<HistoryItem>,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            user_input: String::new(),
            history: AllocRingBuffer::with_capacity(REPL_HISTORY_SIZE),
        }
    }

    pub fn add_history(&mut self, command: &Command, user: bool) {
        match self.history.back_mut() {
            Some(item) if item.command == *command && item.user == user => item.count += 1,
            _ => self.history.push(HistoryItem { command: command.clone(), user, count: 1 }),
        }
    }

    pub fn draw(&mut self, ctx: &Context) -> instructions::Result<Option<Command>> {
        let mut result = Ok(None);
        egui::SidePanel::left("console")
            .resizable(false)
            .min_width(265.0)
            .max_width(265.0)
            .frame(Frame::default().stroke(Stroke::new(2.0, Color32::DARK_GRAY)))
            .show(ctx, |ui| {
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
                            for item in self.history.iter() {
                                body.row(18.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(util::monospace(
                                            if item.command.is_meta() {
                                                "M"
                                            } else if item.user {
                                                "U"
                                            } else {
                                                " "
                                            }
                                        ));
                                    });
                                    row.col(|ui| {
                                        ui.label(util::monospace(&match item.command.opcode() {
                                            None => String::from(""),
                                            Some(opcode) => format!("{}", opcode),
                                        }));
                                    });
                                    row.col(|ui| {
                                        ui.label(util::monospace(&format!("{}", item.command)));
                                    });
                                    row.col(|ui| {
                                        ui.label(util::monospace(
                                            & if item.count == 1 {
                                                String::from("  ")
                                            } else if item.count < 100
                                            {
                                                format!("{}", item.count)
                                            } else { String::from("+ ") }
                                        ));
                                    });
                                });
                            };
                        });
                    });
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
                            if input.lost_focus() {
                                result = Command::parse(self.user_input.as_str().into());
                                if let Ok(Some(command)) = &result {
                                    self.add_history(command, true);
                                }
                                self.user_input.clear();
                            };
                            input.request_focus();
                        });
                    });
            });
        result
    }
}
