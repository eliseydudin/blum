use super::{token::Keyword, Operand, Token, TokenType};
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Chars<'a>,
    tokens: Vec<Token>,
    start_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        let tokens = Vec::new();
        let start_pos = chars.size_hint().1.unwrap_or_default();
        Self {
            tokens,
            chars,
            start_pos,
        }
    }

    pub fn parse(mut self) -> Self {
        loop {
            let char = match self.skip_whitespace() {
                Some(c) => c,
                None => break,
            };

            if char == '"' {
                self.try_string();
            } else if char.is_numeric() {
                self.try_integer(char);
            } else if char.is_alphabetic() {
                self.try_identifier(char);
            } else {
                match self.try_char(char) {
                    Some(tk) => self.tokens.push(tk),
                    None => self
                        .tokens
                        .push(Token::error("unknown character", self.current_position())),
                };
            }
        }

        self
    }

    pub fn skip_whitespace(&mut self) -> Option<char> {
        for c in self.chars.by_ref() {
            if c.is_whitespace() {
                continue;
            }

            return Some(c);
        }

        None
    }

    pub fn try_char(&mut self, ch: char) -> Option<Token> {
        let data = None;
        let token_type = match ch {
            '+' => TokenType::Operand(Operand::Plus),
            '-' => TokenType::Operand(Operand::Minus),
            '*' => TokenType::Operand(Operand::Mult),
            '/' => TokenType::Operand(Operand::Div),
            '%' => TokenType::Operand(Operand::Mod),

            '.' => TokenType::Operand(Operand::Dot),
            ',' => TokenType::Operand(Operand::Coma),
            ':' => TokenType::Operand(Operand::Colon),
            ';' => TokenType::Operand(Operand::Semicolon),

            '(' => TokenType::Operand(Operand::LParen),
            ')' => TokenType::Operand(Operand::RParen),

            '{' => TokenType::Operand(Operand::LFigure),
            '}' => TokenType::Operand(Operand::RFigure),

            '<' => TokenType::Operand(Operand::Less),
            '>' => TokenType::Operand(Operand::More),
            '=' => TokenType::Operand(Operand::Eq),

            '&' => TokenType::Operand(Operand::And),
            '|' => TokenType::Operand(Operand::Or),
            '!' => TokenType::Operand(Operand::Not),

            _ => {
                if ch.is_numeric() {
                    self.try_integer(ch);
                } else if ch.is_alphabetic() {
                    self.try_identifier(ch);
                } else if ch == '"' {
                    self.try_string();
                }

                return None;
            }
        };

        let pos = self.current_position();
        Some(Token {
            token_type,
            data,
            pos,
        })
    }

    pub fn try_integer(&mut self, curr: char) {
        let mut data = String::new();
        data.push(curr);

        while let Some(ch) = self.chars.next() {
            if ch.is_numeric() {
                data.push(ch);
            } else if ch.is_whitespace() {
                break;
            } else {
                if let Some(t) = self.try_char(ch) {
                    let data = Some(data.clone());
                    let token_type = TokenType::Integer;

                    self.tokens.push(self.token(data, token_type));
                    self.tokens.push(t);
                } else {
                    self.tokens.push(Token::error(
                        "unknown integer literal",
                        self.current_position(),
                    ));
                }

                return;
            }
        }

        let token_type = TokenType::Integer;
        let data = Some(data);
        self.tokens.push(self.token(data, token_type))
    }

    pub fn try_identifier(&mut self, curr: char) {
        let mut data = String::new();
        data.push(curr);

        while let Some(ch) = self.chars.next() {
            if ch.is_alphanumeric() {
                data.push(ch);
            } else if ch.is_whitespace() {
                break;
            } else {
                if let Some(t) = self.try_char(ch) {
                    let (data, token_type) = if let Ok(kw) = Keyword::try_from(data.clone()) {
                        (None, TokenType::Keyword(kw))
                    } else {
                        (Some(data), TokenType::Identifier)
                    };

                    self.tokens.push(self.token(data, token_type));
                    self.tokens.push(t);
                } else {
                    self.tokens.push(Token::error(
                        "unknown string literal",
                        self.current_position(),
                    ));
                }

                return;
            }
        }

        let (data, token_type) = if let Ok(kw) = Keyword::try_from(data.clone()) {
            (None, TokenType::Keyword(kw))
        } else {
            (Some(data), TokenType::Identifier)
        };

        self.tokens.push(self.token(data, token_type))
    }

    pub fn try_string(&mut self) {
        let mut data = String::new();
        for ch in self.chars.by_ref() {
            if ch == '"' {
                let token_type = TokenType::String;
                let data = Some(data);
                let tk = self.token(data, token_type);

                self.tokens.push(tk);
                return;
            }

            data.push(ch);
        }

        let token_type = TokenType::Error("'\"' was never closed");
        let data = None;
        self.tokens.push(self.token(data, token_type));
    }

    pub fn finish(self) -> Vec<Token> {
        self.tokens
    }

    pub fn current_position(&self) -> usize {
        let size_hint = self.chars.size_hint();
        self.start_pos - size_hint.1.unwrap_or_default()
    }

    pub fn token(&self, data: Option<String>, token_type: TokenType) -> Token {
        let pos = self.current_position();
        Token {
            data,
            token_type,
            pos,
        }
    }
}
