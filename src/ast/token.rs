#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Dot,
    Minus,
    Plus,
    QuestionMark,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greated,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Break,
    Else,
    False,
    For,
    Fn,
    If,
    Or,
    Return,
    True,
    Let,
    While,

    Eof,
}

impl TryInto<TokenType> for &str {
    type Error = ();

    fn try_into(self) -> Result<TokenType, Self::Error> {
        match self {
            "and" => Ok(TokenType::And),
            "break" => Ok(TokenType::Break),
            "else" => Ok(TokenType::Else),
            "false" => Ok(TokenType::False),
            "for" => Ok(TokenType::For),
            "fn" => Ok(TokenType::Fn),
            "if" => Ok(TokenType::If),
            "or" => Ok(TokenType::Or),
            "return" => Ok(TokenType::Return),
            "true" => Ok(TokenType::True),
            "let" => Ok(TokenType::Let),
            "while" => Ok(TokenType::While),
            _ => Err(()),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub pos: usize,
    pub literal: Option<String>,
}

impl Token {
    #[inline]
    #[must_use]
    pub const fn new(
        ttype: TokenType,
        lexeme: String,
        line: usize,
        pos: usize,
        literal: Option<String>,
    ) -> Self {
        Self {
            ttype,
            lexeme,
            line,
            pos,
            literal,
        }
    }
}
