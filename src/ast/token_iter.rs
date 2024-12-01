use std::cell::Cell;

use super::Token;

#[derive(Debug)]
pub struct TokenIter {
    pub tokens: Vec<Token>,
    pub counter: Cell<usize>,
}

impl TokenIter {
    pub fn new(tokens: Vec<Token>) -> Self {
        let counter = Cell::new(0);
        Self { tokens, counter }
    }

    pub fn next(&self) {
        self.counter.set(self.counter.get() + 1);
    }

    pub fn back(&self) {
        self.counter.set(self.counter.get() - 1);
    }

    pub fn get(&self) -> Option<Token> {
        self.tokens.get(self.counter.get()).cloned()
    }
}
