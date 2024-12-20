use std::fmt;
use std::num::NonZeroUsize;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Or,
    Return,
    True,
    Let,
    While,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: NonZeroUsize,
}

impl Token {
    pub fn new(
        r#type: TokenType,
        lexeme: &str,
        literal: Option<Literal>,
        line: NonZeroUsize,
    ) -> Self {
        Self {
            r#type,
            lexeme: lexeme.to_owned(),
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.r#type, self.lexeme, self.literal)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}