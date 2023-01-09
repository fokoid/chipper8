use std::str::SplitWhitespace;

use crate::parser::Token;

#[derive(Debug)]
pub struct Tokens<'a> {
    raw: SplitWhitespace<'a>,
    next: Option<Token<'a>>,
}

impl<'a> Tokens<'a> {
    pub fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
        self.next.as_ref()
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.next.take();
        self.next = self.raw.next().map(|x| x.into());
        last
    }
}

impl<'a> From<Tokens<'a>> for String {
    fn from(tokens: Tokens) -> Self {
        let tokens: Vec<_> = tokens.map(String::from).collect();
        tokens.join(" ")
    }
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(raw: &'a str) -> Self {
        let mut raw = raw.trim().split_whitespace();
        let next = raw.next().map(|x| x.into());
        Self { raw, next }
    }
}

