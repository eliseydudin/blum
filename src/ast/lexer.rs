use super::{Literal, Token, TokenType};
use core::clone;
use core::str::FromStr as _;
use std::collections::HashMap;
use std::sync::LazyLock;
use TokenType::{
    And, Bang, BangEqual, Comma, Dot, Else, Eof, Equal, EqualEqual, False, Fn, For, Greater,
    GreaterEqual, Identifier, If, LeftBrace, LeftParen, Less, LessEqual, Let, Minus, Number, Or,
    Plus, Return, RightBrace, RightParen, Semicolon, Slash, Star, True, While,
};

static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("and".to_owned(), And);
    map.insert("else".to_owned(), Else);
    map.insert("false".to_owned(), False);
    map.insert("for".to_owned(), For);
    map.insert("fn".to_owned(), Fn);
    map.insert("if".to_owned(), If);
    map.insert("or".to_owned(), Or);
    map.insert("return".to_owned(), Return);
    map.insert("true".to_owned(), True);
    map.insert("let".to_owned(), Let);
    map.insert("while".to_owned(), While);
    map
});

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    #[must_use]
    pub fn new(source: &str) -> Self {
        let src = source.chars();
        let mut source = vec![];
        for ch in src {
            source.push(ch);
        }
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(Eof, "", None, self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let ch = self.advance();
        match ch {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let type_ = if self.matches('=') { BangEqual } else { Bang };
                self.add_token(type_);
            }
            '=' => {
                let type_ = if self.matches('=') { EqualEqual } else { Equal };
                self.add_token(type_);
            }
            '<' => {
                let type_ = if self.matches('=') { LessEqual } else { Less };
                self.add_token(type_);
            }
            '>' => {
                let type_ = if self.matches('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(type_);
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.increment_line(),
            '"' => self.string(),
            '\0' => self.add_token(Eof),
            _ => {
                if ch.is_ascii_digit() {
                    self.number();
                } else if is_alphanumeric(ch) {
                    self.identifier();
                } else {
                    crate::error(self.line, "unexpected character");
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.get(self.current - 1).copied().unwrap_or('\0')
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.get(self.current).copied().unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            '\0'
        } else {
            self.source.get(self.current + 1).copied().unwrap()
        }
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_full_token(type_, None);
    }

    fn add_full_token(&mut self, type_: TokenType, literal: Option<Literal>) {
        let mut lexeme_dyn = String::new();
        for i in self.start..self.current {
            lexeme_dyn.push(self.source[i]);
        }

        let token = Token::new(type_, &lexeme_dyn, literal, self.line);
        self.tokens.push(token);
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return true;
        }
        if expected == self.source.get(self.current).copied().unwrap() {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.increment_line();
            }
            self.advance();
        }
        if self.is_at_end() {
            crate::error(self.line, "unterminated string");
        }
        self.advance();

        let mut literal = String::new();
        for i in (self.start + 1)..(self.current - 1) {
            literal.push(self.source[i]);
        }

        let literal = Literal::String(literal);
        self.add_full_token(TokenType::String, Some(literal));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let mut literal = String::new();
        for i in self.start..self.current {
            literal.push(self.source[i]);
        }

        let literal = Literal::Number(f64::from_str(&literal).unwrap());
        self.add_full_token(Number, Some(literal));
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let mut text = String::new();
        for i in self.start..self.current {
            text.push(self.source[i]);
        }
        let type_ = KEYWORDS
            .get(&text)
            .map_or_else(|| Identifier, clone::Clone::clone);
        self.add_token(type_);
    }

    fn increment_line(&mut self) {
        self.line += 1;
    }
}

const fn is_alphanumeric(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}
