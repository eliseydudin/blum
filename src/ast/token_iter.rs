use std::cell::Cell;

use super::{Token, TokenType};

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

    pub fn expect(&self, expect: impl Into<TokenType>) -> bool {
        let tt: TokenType = expect.into();
        let token = match self.peek() {
            Some(token) => token,
            None => return false,
        };

        token.token_type == tt
    }

    pub fn expect_and_progress(&self, expect: impl Into<TokenType>) -> bool {
        let result = self.expect(expect);
        self.counter.set(self.counter.get() + 1);
        result
    }
}
