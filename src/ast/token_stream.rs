use super::token::{Token, TokenType};

pub struct TokenStream {
    source: Vec<char>,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl TokenStream {
    pub fn new(source: &str) -> Self {
        let source: Vec<char> = source.chars().collect();

        Self {
            source,
            tokens: vec![],

            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.get(self.current).cloned()
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.current + 1).cloned()
    }

    pub fn add_token(&mut self, ttype: TokenType, data: Option<String>) {
        let mut lexeme = String::new();
        let text = &self.source[self.start..self.current];

        for ch in text {
            lexeme.push(ch.clone())
        }

        let new_token = Token::new(ttype, lexeme, self.line, data);
        self.tokens.push(new_token);
    }

    pub fn add_token_small(&mut self, ttype: TokenType) {
        self.add_token(ttype, None);
    }

    pub fn scan_token(&mut self) -> () {
        let next = match self.advance() {
            Some(n) => n,
            None => {
                self.add_token_small(TokenType::Eof);
                return;
            }
        };

        match next {
            '(' => self.add_token_small(TokenType::LeftParen),
            ')' => self.add_token_small(TokenType::RightParen),
            '{' => self.add_token_small(TokenType::LeftBrace),
            '}' => self.add_token_small(TokenType::RightBrace),
            ',' => self.add_token_small(TokenType::Comma),
            '.' => self.add_token_small(TokenType::Dot),
            '-' => self.add_token_small(TokenType::Minus),
            '+' => self.add_token_small(TokenType::Plus),
            ';' => self.add_token_small(TokenType::Semicolon),
            '*' => self.add_token_small(TokenType::Star),
            ':' => self.add_token_small(TokenType::Colon),
            '?' => self.add_token_small(TokenType::QuestionMark),
            '!' => {
                if self.peek().is_some_and(|s| s == '=') {
                    self.current += 1;
                    self.add_token_small(TokenType::BangEqual);
                } else {
                    self.add_token_small(TokenType::Bang);
                }
            }
            '=' => {
                if self.peek().is_some_and(|s| s == '=') {
                    self.current += 1;
                    self.add_token_small(TokenType::EqualEqual);
                } else {
                    self.add_token_small(TokenType::Equal);
                }
            }
            '>' => {
                if self.peek().is_some_and(|s| s == '=') {
                    self.current += 1;
                    self.add_token_small(TokenType::GreaterEqual);
                } else {
                    self.add_token_small(TokenType::Greated);
                }
            }
            '<' => {
                if self.peek().is_some_and(|s| s == '=') {
                    self.current += 1;
                    self.add_token_small(TokenType::LessEqual);
                } else {
                    self.add_token_small(TokenType::Less);
                }
            }
            '/' => {
                if self.peek().is_some_and(|s| s == '/') {
                    while self.peek().is_some_and(|s| s != '\n') {
                        self.advance();
                    }
                } else {
                    self.add_token_small(TokenType::Slash);
                }
            }
            '\n' => self.line += 1,
            n if n.is_whitespace() => (),
            '"' => self.try_string(),
            n if n.is_ascii_digit() => self.try_number(),
            n if n.is_ascii_alphabetic() => self.try_identifier(),
            unknown => {
                println!("Unknown character {unknown}")
            }
        }
    }

    pub fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn try_string(&mut self) -> () {
        while let Some(next) = self.peek() {
            if next == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_eof() {
            println!("String was never closed");
            return;
        }

        self.advance();
        let mut str = String::new();
        let source = &self.source[self.start + 1..self.current - 1];
        for ch in source {
            str.push(ch.clone());
        }

        self.add_token(TokenType::String, Some(str));
    }

    pub fn try_number(&mut self) -> () {
        todo!()
    }

    pub fn try_identifier(&mut self) -> () {
        todo!()
    }
}
