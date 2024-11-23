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

            if char == '-' {
                let token_type = TokenType::Operand(Operand::Minus);
                let data = None;
                tokens.push(Token { token_type, data });
                continue;
            } else if char == '+' {
                let token_type = TokenType::Operand(Operand::Plus);
                let data = None;
                tokens.push(Token { token_type, data });
                continue;
            }

            if char.is_numeric() {
                let mut data = String::new();
                data.push(char);

                while let Some(ch) = chars.next() {
                    if ch.is_numeric() {
                        data.push(ch);
                    } else {
                        //tokens.push(Token::error("unknown integer"));
                        break;
                    }
                }

                let data = Some(data);
                let token_type = TokenType::Integer;

                tokens.push(Token { data, token_type })
            }
        }

        return tokens;
    }
}
