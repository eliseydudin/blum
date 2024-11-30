use super::{token::Keyword, Error, Expr, Lexer, Operand, Result, Token, TokenIter, TokenType};
use crate::error;
use std::collections::HashMap;

pub struct Parser {
    pub tokens: TokenIter,
    pub ast: Vec<Expr>,
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        let input: String = input.into();

        let tokens = Lexer::new(input.chars()).parse().finish();
        let tokens = TokenIter::new(tokens);
        let ast = vec![];

        Self { ast, tokens }
    }

    /// Parse the [`Self::tokens`] into a `Vec<Expr>`
    /// Returns errors if any were found
    pub fn parse(&mut self) -> Vec<Error> {
        let mut errors = vec![];

        while let Some(token) = self.tokens.next() {
            match self.parse_next(token) {
                Ok(e) => self.ast.push(e),
                Err(e) => errors.push(e),
            }
        }

        errors
    }

    /// Attempt to parse the given tokens
    pub fn parse_next(&mut self, token: Token) -> Result<Expr> {
        match token.token_type {
            TokenType::Keyword(kw) => self.try_keyword(kw),
            TokenType::Error(err) => error!(err).wrap(),
            _ => error!().wrap(),
        }
    }

    /// Matches the given keyword to attempt building the AST
    pub fn try_keyword(&mut self, kw: Keyword) -> Result<Expr> {
        match kw {
            Keyword::Fn => self.try_function(),
            _ => error!("currently only `Keyword::Fn` is supported").wrap(),
        }
    }

    /// Try parsing a function definition
    /// ```blum
    /// fn main() {
    ///     // nothing here
    /// }
    /// ```
    /// `try_function` will build the following AST:
    /// ```text
    /// Function {
    ///     name: "main",
    ///     rettype: "void",
    ///     params: {},
    ///     body: Block(
    ///        [],
    ///     ),
    /// },
    /// ```
    ///
    pub fn try_function(&mut self) -> Result<Expr> {
        let identifier = self
            .tokens
            .expect_and_progress(TokenType::Identifier)
            .ok_or(Error::EOF(TokenType::Identifier))?;

        if !identifier.0 {
            return Error::Expect {
                expected: TokenType::Identifier,
                found: identifier.1.token_type,
            }
            .wrap();
        }

        // _lparen should just be ignored
        let _lparen = self
            .tokens
            .expect_and_progress(Operand::LParen)
            .ok_or(Error::EOF(TokenType::Operand(Operand::LParen)))?;

        // we can do .unwrap since [`TokenType::Identifier`] always has some data
        let name = identifier.1.data.unwrap();
        let params = self.try_type_map(Operand::RParen, Operand::Coma)?;
        let rettype = self
            .try_function_return_type()?
            .unwrap_or("void".to_owned());
        let body = Box::new(self.try_block()?);

        Ok(Expr::Function {
            name,
            rettype,
            params,
            body,
        })
    }

    /// Attempt to parse a block
    pub fn try_block(&mut self) -> Result<Expr> {
        eprintln!("warning! `try_block` currently does nothing!");
        loop {
            let token = self.tokens.next();
            match token {
                Some(next) => {
                    if next.token_type == Operand::RFigure.into() {
                        break;
                    }
                }
                None => return error!("`try_block` never found `end`!").wrap(),
            }
        }
        Ok(Expr::Block(vec![]))
    }

    /// Attempt to parse a type map.
    /// A type map is an expression which looks like this:
    /// ```blum
    /// <any> expr: type <sep> expr2: type2 <sep> ... <end>
    /// ```
    /// This could be used in functions
    /// ```blum
    /// fn test(a: i32, b: f64) ...
    /// ```
    /// Or in type definition
    /// ```blum
    /// type Sum = {a: i32, b: f64}; // sep = "," end = "}"
    /// type Alg = {a: i32 | b: f64}; // sep = "|" end = "}"
    /// ```
    pub fn try_type_map(&mut self, end: Operand, sep: Operand) -> Result<HashMap<String, String>> {
        let mut result = HashMap::new();

        loop {
            let token = self.tokens.next();
            match token {
                Some(next) => {
                    if next.token_type == end.into() {
                        break;
                    }

                    let to_insert = self.try_type_map_next(next)?;
                    result.insert(to_insert.0, to_insert.1);

                    let sep_t = self
                        .tokens
                        .expect_and_progress(sep)
                        .ok_or(Error::EOF(sep.into()))?;

                    if sep_t.0 {
                        continue;
                    } else if sep_t.1.token_type == end.into() {
                        return Ok(result);
                    }
                }
                None => return error!("`try_type_map` never found `end`!").wrap(),
            }
        }

        Ok(result)
    }

    /// Attempt to parse the next element of the current type map
    pub fn try_type_map_next(&mut self, token: Token) -> Result<(String, String)> {
        match self.tokens.expect_and_progress(Operand::Colon) {
            Some(data) => {
                if !data.0 {
                    return Error::Expect {
                        expected: Operand::Colon.into(),
                        found: data.1.token_type,
                    }
                    .wrap();
                }
            }
            None => return Error::EOF(Operand::Colon.into()).wrap(),
        };

        let ptype = match self.tokens.expect_and_progress(TokenType::Identifier) {
            Some(data) => {
                if data.0 {
                    data.1
                } else {
                    return Error::Expect {
                        expected: TokenType::Identifier,
                        found: data.1.token_type,
                    }
                    .wrap();
                }
            }
            None => return Error::EOF(TokenType::Identifier).wrap(),
        };

        Ok((token.data.unwrap(), ptype.data.unwrap()))
    }

    /// Try to parse the return type of the function.
    /// It will return `Ok(None)` if the function's return type is void.
    /// `fn main() ...` -> `None`
    /// `fn test(a: i32, b: f64) -> i32 ...` -> `Ok("i32")`
    pub fn try_function_return_type(&mut self) -> Result<Option<String>> {
        let token = self.tokens.current().ok_or(error!("found EOF!"))?;

        if token.token_type == Operand::LFigure.into() {
            return Ok(None);
        } else if token.token_type == Operand::Minus.into() {
            self.tokens.progress();
            let more_token = self.tokens.next().ok_or(error!("found EOF!"))?;
            let type_identifier = self.tokens.next().ok_or(error!("found EOF!"))?;

            if more_token.token_type == Operand::More.into()
                && type_identifier.token_type == TokenType::Identifier
            {
                return Ok(Some(type_identifier.data.unwrap()));
            } else {
                return error!("unexpected token found in the -> operator!").wrap();
            }
        }

        error!().wrap()
    }

    /// Parse an expression until a token with type `until` is met.
    /// For example:
    /// ```blum
    /// return a + 20;
    /// ```
    /// It will go from `a` to 20
    pub fn try_value_until(&mut self, until: Operand) -> Result<Expr> {
        error!().wrap()
    }
}
