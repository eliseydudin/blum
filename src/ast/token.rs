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

impl Operand {
    pub fn is_mathematical(&self) -> bool {
        match self {
            Self::Plus | Self::Minus | Self::Mult | Self::Div | Self::Mod => true,
            _ => false,
        }
    }
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

impl TokenType {
    pub fn is_mathematical_op(&self) -> bool {
        match self {
            Self::Operand(op) => op.is_mathematical(),
            _ => false,
        }
    }

    pub fn to_operand(self) -> Operand {
        match self {
            Self::Operand(op) => op,
            _ => unreachable!(),
        }
    }
}

impl From<Operand> for TokenType {
    fn from(val: Operand) -> Self {
        TokenType::Operand(val)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub data: Option<String>,
    pub pos: usize,
}

impl Token {
    pub const fn error(err: &'static str, pos: usize) -> Token {
        Token {
            token_type: TokenType::Error(err),
            data: None,
            pos,
        }
    }
}
