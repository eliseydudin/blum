#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
    Fn,
    If,
    Return,
}

impl TryFrom<String> for Keyword {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "fn" => Ok(Self::Fn),
            "if" => Ok(Self::If),
            "return" => Ok(Self::Return),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    Plus,  // +
    Minus, // -
    Mult,  // *
    Div,   // /
    Mod,   // %

    Dot,       // .
    Coma,      // ,
    Colon,     // :
    Semicolon, // ;

    LParen, // (
    RParen, // )

    LFigure, // {
    RFigure, // }

    Less, // <
    More, // >
    Eq,   // =

    And, // &
    Or,  // |
    Not, // !
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Integer,
    String,
    Identifier,
    Operand(Operand),
    Keyword(Keyword),
    Error(&'static str),
}

impl Into<TokenType> for Operand {
    fn into(self) -> TokenType {
        TokenType::Operand(self)
    }
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
