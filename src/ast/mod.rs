#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Plus,
    Minus,
}

#[derive(Clone, Debug)]
pub enum TokenType {
    Integer,
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

        'main_loop: loop {
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
                let mut data = String::new();
                data.push(char);

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
                        }

                        continue 'main_loop;
                    }
                }

                let data = Some(data);
                let token_type = TokenType::Integer;

                tokens.push(Token { data, token_type })
            }
        }

        return tokens;
    }

    pub fn try_char(ch: char) -> Option<Token> {
        let data = None;
        let token_type = match ch {
            '+' => TokenType::Operand(Operand::Plus),
            '-' => TokenType::Operand(Operand::Minus),
            _ => return None,
        };

        return Some(Token { token_type, data });
    }
}
