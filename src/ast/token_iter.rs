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

    pub fn expect(&self, expect: impl Into<TokenType>) -> Option<(bool, Token)> {
        let expect: TokenType = expect.into();
        match self.peek() {
            Some(token) => {
                if token.token_type == expect {
                    Some((true, token))
                } else {
                    Some((false, token))
                }
            }
            None => None,
        }
    }

    pub fn expect_and_progress(&self, expect: impl Into<TokenType>) -> Option<(bool, Token)> {
        let result = self.expect(expect);
        self.counter.set(self.counter.get() + 1);
        result
    }
}
