#[derive(Debug, Clone)]
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
    Fun,
    If,
    Or,
    Return,
    True,
    Let,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<String>,
}

impl Token {
    pub const fn new(
        ttype: TokenType,
        lexeme: String,
        line: usize,
        literal: Option<String>,
    ) -> Self {
        Self {
            ttype,
            lexeme,
            line,
            literal,
        }
    }
}
