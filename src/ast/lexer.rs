use super::Token;

pub struct Lexer<'a> {
    lines: Vec<&'a str>,
    tokens: Vec<Token>,

    pos: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(str: &'a str) -> Self {
        let lines: Vec<&str> = str.lines().collect();
        let tokens = vec![];

        Self {
            lines,
            tokens,
            pos: 0,
            line: 0,
        }
    }

    pub fn parse(self) -> Self {
        todo!()
    }

    pub fn finish(self) -> Vec<Token> {
        self.tokens
    }
}
