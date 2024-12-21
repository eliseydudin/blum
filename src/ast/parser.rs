use super::TokenType::{
    Bang, BangEqual, Else, Eof, Equal, EqualEqual, False, Fn, For, Greater, GreaterEqual,
    Identifier, If, LeftBrace, LeftParen, Less, LessEqual, Let, Minus, Number, Or, Plus, Return,
    RightBrace, RightParen, Semicolon, Slash, Star, String, True, While,
};
use super::{Expr, Stmt};
use super::{Literal, Token, TokenType};
use crate::error;
use anyhow::{anyhow, Result};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        loop {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            if self.is_at_end() {
                break;
            }
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.matches(&[Let]) {
            if let Ok(stmt) = self.var_declaration() {
                Some(stmt)
            } else {
                self.synchronize();
                None
            }
        } else if let Ok(stmt) = self.statement() {
            Some(stmt)
        } else {
            self.synchronize();
            None
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume(&Identifier, "expected variable name.")?;
        let initializer = if self.matches(&[Equal]) {
            self.expression().ok()
        } else {
            None
        };
        self.consume(&Semicolon, "expected ';' after variable declaration.")?;
        Ok(Stmt::Let(name, initializer))
    }

    fn fn_statement(&mut self) -> Result<Stmt> {
        let name = self.consume(&Identifier, "expected function name")?;
        // todo read function params
        self.consume(&LeftParen, "expected '(' after function identifier")?;
        self.consume(&RightParen, "function parameter list never closed")?;
        let block = self.block()?;

        self.advance();

        Ok(Stmt::Fn(name, vec![], Box::new(block[0].clone())))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.matches(&[For]) {
            self.for_statement()
        } else if self.matches(&[If]) {
            self.if_statement()
        } else if self.matches(&[While]) {
            self.while_statement()
        } else if self.matches(&[LeftBrace]) {
            Ok(Stmt::Block(self.block()?))
        } else if self.matches(&[Fn]) {
            self.fn_statement()
        } else {
            self.expression_statement()
        }
    }

    fn for_statement(&mut self) -> Result<Stmt> {
        self.consume(&LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.matches(&[Semicolon]) {
            None
        } else if self.matches(&[Let]) {
            self.var_declaration().ok()
        } else {
            self.expression_statement().ok()
        };
        let condition = if self.check(&Semicolon) {
            Expr::Literal(Literal::Bool(true))
        } else {
            self.expression()?
        };

        let increment = if self.check(&RightParen) {
            None
        } else {
            self.expression().ok()
        };
        self.consume(&RightParen, "expected ')' after for clauses")?;
        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(increment)]);
        }
        body = Stmt::While(condition, Box::new(body));
        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![initializer, body]);
        }
        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume(&LeftParen, "expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(&RightParen, "expected ')' after the condition")?;
        let body = self.statement()?;
        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(&LeftParen, "expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(&RightParen, "expected ')' after if condition")?;
        let then_branch = self.statement()?;
        let else_branch = if self.matches(&[Else]) {
            self.statement().ok()
        } else {
            None
        };
        Ok(Stmt::If(
            condition,
            Box::new(then_branch),
            Box::new(else_branch),
        ))
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        loop {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            if self.check(&RightBrace) || self.is_at_end() {
                break;
            }
        }
        //self.consume(&RightBrace, "Expect ';' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&Semicolon, "expected ';' after expression")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?;
        if self.matches(&[Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable(name) = expr {
                Ok(Expr::Assign(name, Box::new(value)))
            } else {
                error(equals.line, "invalid assignment target");
                Ok(expr)
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;
        while self.matches(&[Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;
        while self.matches(&[Or]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while self.matches(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while self.matches(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.matches(&[Plus, Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.matches(&[Slash, Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.matches(&[Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(operator, Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.matches(&[False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.matches(&[True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.matches(&[Number, String]) {
            return Ok(Expr::Literal(match self.previous().literal {
                Some(l) => l,
                None => Literal::Nil,
            }));
        }
        if self.matches(&[Identifier]) {
            return Ok(Expr::Variable(self.previous()));
        }
        if self.matches(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(&RightParen, "expected `)` after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        crate::error_at_token(&self.peek(), "expected expression");
        Err(anyhow!("Parse error"))
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().r#type == Semicolon {
                return;
            }
            match self.peek().r#type {
                Let | For | If | While | Return => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for r#type in types {
            if self.check(r#type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, r#type: &TokenType, message: &str) -> Result<Token> {
        if self.check(r#type) {
            Ok(self.advance())
        } else {
            crate::error_at_token(&self.peek(), message);
            Err(anyhow!("Parse error"))
        }
    }

    fn check(&self, r#type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().r#type == r#type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::ast::{lexer::Lexer, Stmt};

    use super::Parser;

    #[test]
    fn basic_parser_test() {
        let source = include_str!("../../test.blum");
        let mut lexer = Lexer::new(source.to_owned());
        let mut parser = Parser::new(lexer.scan_tokens());
        let ast = parser.parse();

        match ast[0] {
            Stmt::Fn(_, _, _) => (),
            _ => panic!(),
        }

        match ast[1] {
            Stmt::Let(_, _) => (),
            _ => panic!(),
        }
    }
}
