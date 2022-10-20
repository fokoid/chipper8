use super::config;

#[derive(Debug)]
pub struct Stack {
    pub data: [u16; config::STACK_SIZE],
    pub pointer: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0; config::STACK_SIZE],
            pointer: 0,
        }
    }

    pub fn pop(&mut self) -> u16 {
        if self.pointer == 0 {
            panic!("pop() on empty stack");
        };
        let value = self.data[self.pointer];
        self.pointer -= 1;
        value
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