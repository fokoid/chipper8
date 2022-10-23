use serde::{Deserialize, Serialize};

use super::config;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Stack {
    pub data: Vec<u16>,
    pub pointer: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: vec![0; config::STACK_SIZE],
            pointer: 0,
        }
    }

    pub fn pop(&mut self) -> u16 {
        if self.pointer == 0 {
            panic!("pop() on empty stack");
        };
        self.pointer -= 1;
        self.data[self.pointer]
    }

    pub fn push(&mut self, value: u16) {
        if self.pointer == config::STACK_SIZE {
            panic!("push({}) on full stack", value);
        }
        self.data[self.pointer] = value;
        self.pointer += 1;
    }

    pub fn reset(&mut self) {
        self.data.fill(0);
        self.pointer = 0;
    }
}