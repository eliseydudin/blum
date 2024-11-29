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

    pub fn next(&self) -> Option<Token> {
        let tk = self.tokens.get(self.counter.get()).cloned();
        self.counter.set(self.counter.get() + 1);
        tk
    }

    pub fn peek(&self) -> Option<Token> {
        self.tokens.get(self.counter.get() + 1).cloned()
    }

    pub fn current(&self) -> Option<Token> {
        self.tokens.get(self.counter.get()).cloned()
    }
}
