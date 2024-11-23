use super::{Operand, Token, TokenType};
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Chars<'a>,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        let tokens = Vec::new();
        Self { tokens, chars }
    }

    pub fn parse(&mut self) {
        loop {
            let char = match self.skip_whitespace() {
                Some(c) => c,
                None => break,
            };

            if let Some(tk) = Self::try_char(char) {
                self.tokens.push(tk);
                continue;
            }

            if char.is_numeric() {
                self.try_integer(char);
            } else if char.is_alphabetic() {
                self.try_identifier(char);
            }
        }
    }

    pub fn skip_whitespace(&mut self) -> Option<char> {
        while let Some(c) = self.chars.next() {
            if c.is_whitespace() {
                continue;
            }

            return Some(c);
        }

        None
    }

    pub fn try_char(ch: char) -> Option<Token> {
        let data = None;
        let token_type = match ch {
            '+' => TokenType::Operand(Operand::Plus),
            '-' => TokenType::Operand(Operand::Minus),
            '*' => TokenType::Operand(Operand::Mult),
            '/' => TokenType::Operand(Operand::Div),
            '%' => TokenType::Operand(Operand::Mod),

            '.' => TokenType::Operand(Operand::Dot),

            '(' => TokenType::Operand(Operand::LParen),
            ')' => TokenType::Operand(Operand::RParen),

            _ => return None,
        };

        return Some(Token { token_type, data });
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
                if let Some(t) = Self::try_char(ch) {
                    let data = Some(data.clone());
                    let token_type = TokenType::Integer;

                    self.tokens.push(Token { data, token_type });
                    self.tokens.push(t);
                } else {
                    self.tokens.push(Token::error("unknown integer literal"));
                }

                return;
            }
        }

        let token_type = TokenType::Integer;
        let data = Some(data);
        self.tokens.push(Token { token_type, data })
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
                if let Some(t) = Self::try_char(ch) {
                    let data = Some(data.clone());
                    let token_type = TokenType::Identifier;

                    self.tokens.push(Token { data, token_type });
                    self.tokens.push(t);
                } else {
                    self.tokens.push(Token::error("unknown string literal"));
                }

                return;
            }
        }

        let token_type = TokenType::Identifier;
        let data = Some(data);
        self.tokens.push(Token { token_type, data })
    }

    pub fn finish(self) -> Vec<Token> {
        self.tokens
    }
}
