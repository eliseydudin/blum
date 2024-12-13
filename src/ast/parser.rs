use super::{
    expr, statement as stmt,
    token::{self as tk, TokenType},
    token_stream::TokenStream,
};

pub struct Parser {
    tokens: Vec<tk::Token>,
    current: usize,
    statements: Vec<stmt::Statement>,
}

impl Parser {
    pub fn new(lexer: TokenStream) -> Self {
        let tokens = lexer.lex();
        Self {
            tokens,
            current: 0,
            statements: vec![],
        }
    }

    pub fn parse(self) -> Vec<stmt::Statement> {
        self.statements
    }

    pub fn peek(&self) -> Option<tk::Token> {
        self.tokens.get(self.current + 1).cloned()
    }

    pub fn expect(&mut self, ttype: TokenType) -> Option<tk::Token> {
        let tk = self.advance();
        tk.and_then(|t| if t.ttype == ttype { Some(t) } else { None })
    }

    pub fn advance(&mut self) -> Option<tk::Token> {
        self.current += 1;
        self.tokens.get(self.current - 1).cloned()
    }

    pub fn read_expression(&mut self) -> Option<expr::Expr> {
        todo!()
    }

    pub fn try_let(&mut self) -> Option<stmt::Let> {
        let _bind = self.expect(TokenType::Let)?;
        let name = self.expect(TokenType::Identifier)?;
        let _eq = self.expect(TokenType::Equal)?;
        let value = self.read_expression()?;

        let let_statement = stmt::Let {
            name: name.lexeme,
            value,
        };

        Some(let_statement)
    }
}
