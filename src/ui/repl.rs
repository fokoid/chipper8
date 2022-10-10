use egui::{Color32, Context, Frame, Stroke, TextStyle, Ui};
use egui::style::Margin;
use egui::widgets::TextEdit;
use egui_extras::{Size, TableBuilder, TableRow};
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};

use chipper8::instructions::{self, Command};

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
                table::build(
                    ui,
                    vec![10.0, 40.0, 160.0, 20.0],
                    &self.items,
                )
            });
    }
}