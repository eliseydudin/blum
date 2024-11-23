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
}
