use super::{Literal, Token, TokenType};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;
use TokenType::{
    And, Bang, BangEqual, Comma, Dot, Else, Eof, Equal, EqualEqual, False, Fn, For, Greater,
    GreaterEqual, Identifier, If, LeftBrace, LeftParen, Less, LessEqual, Let, Minus, Number, Or,
    Plus, Return, RightBrace, RightParen, Semicolon, Slash, Star, True, While,
};

static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("and".to_owned(), And);
    m.insert("else".to_owned(), Else);
    m.insert("false".to_owned(), False);
    m.insert("for".to_owned(), For);
    m.insert("fn".to_owned(), Fn);
    m.insert("if".to_owned(), If);
    m.insert("or".to_owned(), Or);
    m.insert("return".to_owned(), Return);
    m.insert("true".to_owned(), True);
    m.insert("let".to_owned(), Let);
    m.insert("while".to_owned(), While);
    m
});

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
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
        let c = self.advance();
        match c {
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
                self.add_token(type_)
            }
            '=' => {
                let type_ = if self.matches('=') { EqualEqual } else { Equal };
                self.add_token(type_)
            }
            '<' => {
                let type_ = if self.matches('=') { LessEqual } else { Less };
                self.add_token(type_)
            }
            '>' => {
                let type_ = if self.matches('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(type_)
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
            _ => {
                if c.is_digit(10) {
                    self.number()
                } else if is_alphanumeric(c) {
                    self.identifier()
                } else {
                    crate::error(self.line, "unexpected character")
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_full_token(type_, None);
    }

    fn add_full_token(&mut self, type_: TokenType, literal: Option<Literal>) {
        let token = Token::new(
            type_,
            &self.source[self.start..self.current],
            literal,
            self.line,
        );
        self.tokens.push(token);
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return true;
        }
        if expected == self.source.chars().nth(self.current).unwrap() {
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
        let literal = Literal::String(self.source[(self.start + 1)..(self.current - 1)].to_owned());
        self.add_full_token(TokenType::String, Some(literal));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let literal =
            Literal::Number(f64::from_str(&self.source[(self.start)..(self.current)]).unwrap());
        self.add_full_token(Number, Some(literal));
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[(self.start)..(self.current)];
        let type_ = KEYWORDS
            .get(text)
            .map_or_else(|| Identifier, std::clone::Clone::clone);
        self.add_token(type_);
    }

    fn increment_line(&mut self) {
        self.line += 1;
    }
}

const fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
