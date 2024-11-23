use std::str::Chars;

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Plus,  // +
    Minus, // -
    Mult,  // *
    Div,   // /
    Mod,   // %

    Dot, // .

    LParen, // (
    RParen, // )
}

#[derive(Clone, Debug)]
pub enum TokenType {
    Integer,
    Identifier,
    Operand(Operand),
    Error(&'static str),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub data: Option<String>,
}

impl Token {
    pub const fn error(err: &'static str) -> Token {
        Token {
            token_type: TokenType::Error(err),
            data: None,
        }
    }

    pub fn parse_string(source: impl Into<String>) -> Vec<Token> {
        let mut tokens = Vec::new();
        let source: String = source.into();
        let mut chars = source.chars();

        loop {
            let mut char = match chars.next() {
                Some(d) => d,
                None => break,
            };

            if char.is_whitespace() {
                while let Some(c) = chars.next() {
                    if !c.is_whitespace() {
                        char = c;
                        break;
                    }
                }
            }

            if let Some(tk) = Self::try_char(char) {
                tokens.push(tk);
                continue;
            }

            if char.is_numeric() {
                Self::try_integer(&mut tokens, &mut chars, char);
            } else if char.is_alphabetic() {
                Self::try_identifier(&mut tokens, &mut chars, char);
            }
        }

        return tokens;
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

    pub fn try_integer(tokens: &mut Vec<Token>, chars: &mut Chars<'_>, init_char: char) {
        let mut data = String::new();
        data.push(init_char);

        while let Some(ch) = chars.next() {
            if ch.is_numeric() {
                data.push(ch);
            } else if ch.is_whitespace() {
                break;
            } else {
                if let Some(t) = Self::try_char(ch) {
                    let data = Some(data.clone());
                    let token_type = TokenType::Integer;

                    tokens.push(Token { data, token_type });
                    tokens.push(t);
                } else {
                    tokens.push(Token::error("unknown integer literal"));
                }

                return;
            }
        }

        let token_type = TokenType::Integer;
        let data = Some(data);
        tokens.push(Token { token_type, data })
    }

    pub fn try_identifier(tokens: &mut Vec<Token>, chars: &mut Chars<'_>, init_char: char) {
        let mut data = String::new();
        data.push(init_char);

        while let Some(ch) = chars.next() {
            if ch.is_alphanumeric() {
                data.push(ch);
            } else if ch.is_whitespace() {
                break;
            } else {
                if let Some(t) = Self::try_char(ch) {
                    let data = Some(data.clone());
                    let token_type = TokenType::Identifier;

                    tokens.push(Token { data, token_type });
                    tokens.push(t);
                } else {
                    tokens.push(Token::error("unknown string literal"));
                }

                return;
            }
        }

        let token_type = TokenType::Identifier;
        let data = Some(data);
        tokens.push(Token { token_type, data })
    }
}
