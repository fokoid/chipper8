use serde::{Deserialize, Serialize};

use super::config;
use super::types::Address;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Stack {
    pub data: Vec<Option<Address>>,
    pub pointer: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: vec![None; config::STACK_SIZE],
            pointer: 0,
        }
    }

    pub fn pop(&mut self) -> Address {
        if self.pointer == 0 {
            panic!("pop() on empty stack");
        };
        self.pointer -= 1;
        self.data[self.pointer].take().unwrap()
    }

    pub fn push(&mut self, address: Address) {
        if self.pointer == config::STACK_SIZE {
            panic!("push({}) on full stack", address);
        }
        self.data[self.pointer] = Some(address);
        self.pointer += 1;
    }

    pub fn reset(&mut self) {
        self.data.fill(None);
        self.pointer = 0;
    }
}