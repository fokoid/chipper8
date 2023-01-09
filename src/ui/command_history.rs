use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};

use crate::command::Command;
use crate::machine::OpCode;

// hard coded based on current (also hard coded) UI element sizes
const REPL_HISTORY_SIZE: usize = 16;

pub struct HistoryItem {
    pub command: Command,
    pub opcode: Option<OpCode>,
    pub user: bool,
    pub count: usize,
}

pub struct CommandHistory {
    pub items: AllocRingBuffer<HistoryItem>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self { items: AllocRingBuffer::with_capacity(REPL_HISTORY_SIZE) }
    }

    pub fn append(&mut self, command: &Command, opcode: Option<OpCode>, user: bool) {
        match self.items.back_mut() {
            Some(item) if item.command == *command && item.user == user => item.count += 1,
            _ => self.items.push(HistoryItem { command: command.clone(), opcode, user, count: 1 }),
        }
    }
}