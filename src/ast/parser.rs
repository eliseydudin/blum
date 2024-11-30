use super::{
    expr::Value, token::Keyword, EofFoundUtils, Error, ExpectUtils, Expr, Lexer, Operand, Result,
    Token, TokenIter, TokenType,
};
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
            .expect_ext(TokenType::Identifier)?;

        // _lparen should just be ignored
        self.tokens
            .expect_and_progress(Operand::LParen)
            .expect_ext(Operand::LParen.into())?;

        // we can do .unwrap since [`TokenType::Identifier`] always has some data
        let name = identifier.data.unwrap();
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

    // Helper for [`Self::try_type_map`]
    pub fn try_type_map_helper(
        &mut self,
        result: &mut HashMap<String, String>,
        next: Option<Token>,
        end: Operand,
        sep: Operand,
    ) -> Result<bool> {
        if next.is_none() {
            return error!("`try_type_map` never found `end`!").wrap();
        }

        let tk = next.unwrap();

        if tk.token_type == end.into() {
            return Ok(false);
        }

        let to_insert = self.try_type_map_next(tk)?;
        result.insert(to_insert.0, to_insert.1);

        let sep_t = self
            .tokens
            .expect_and_progress(sep)
            .ok_or(Error::Eof(sep.into()))?;

        if sep_t.0 {
            Ok(true)
        } else {
            /*if sep_t.1.token_type == end.into()*/
            Ok(false)
        }
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
            let cont = self.try_type_map_helper(&mut result, token, end, sep)?;
            if !cont {
                break;
            }
        }

        Ok(result)
    }

    /// Attempt to parse the next element of the current type map
    pub fn try_type_map_next(&mut self, token: Token) -> Result<(String, String)> {
        // Just expect a colon
        self.tokens
            .expect_and_progress(Operand::Colon)
            .expect_ext(Operand::Colon.into())?;

        let ptype = self
            .tokens
            .expect_and_progress(TokenType::Identifier)
            .expect_ext(TokenType::Identifier)?;

        Ok((token.data.unwrap(), ptype.data.unwrap()))
    }

    /// Try to parse the return type of the function.
    /// It will return `Ok(None)` if the function's return type is void.
    /// `fn main() ...` -> `None`
    /// `fn test(a: i32, b: f64) -> i32 ...` -> `Ok("i32")`
    pub fn try_function_return_type(&mut self) -> Result<Option<String>> {
        let token = self.tokens.current().eof_error()?;

        if token.token_type == Operand::LFigure.into() {
            return Ok(None);
        } else if token.token_type == Operand::Minus.into() {
            self.tokens.progress();
            let more_token = self.tokens.next().eof_error()?;
            let type_identifier = self.tokens.next().eof_error()?;

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
    pub fn try_value_until(&mut self, end: Option<Operand>) -> Result<Expr> {
        let token = self.tokens.next().eof_error()?;
        match token.token_type {
            TokenType::Identifier => {
                let start = &mut token.data.unwrap();
                let identifier = Expr::variable_ref(self.try_value_element_access(start)?);
                let next = match self.tokens.next() {
                    Some(tk) => tk,
                    None => return Ok(identifier),
                };

                if next.token_type.is_mathematical_op() {
                    let expr = self.try_binop(identifier, next.token_type.to_operand())?;
                    return Ok(expr);
                }

                Ok(Expr::Null)
            }
            TokenType::Operand(op) => {
                if Some(op) == end {
                    return Ok(Expr::Null);
                }

                Ok(self.try_operand(op)?)
            }
            _ => error!("unexpected token!").wrap(),
        }
    }

    // Used to parse expressions like `foo.bar.silly.etc`
    pub fn try_value_element_access(&mut self, start: &mut String) -> Result<String> {
        let next_tk = self.tokens.next();
        match next_tk {
            Some(data) => {
                if data.token_type != Operand::Dot.into() {
                    Ok(start.clone())
                } else {
                    let next_tk = self
                        .tokens
                        .expect_and_progress(TokenType::Identifier)
                        .expect_ext(TokenType::Identifier)?;

                    *start += &(next_tk.data.unwrap());
                    self.try_value_element_access(start)?;

                    Ok(start.clone())
                }
            }
            None => Error::Eof(Operand::Dot.into()).wrap(),
        }
    }

    pub fn try_call_parameters(&mut self) -> Result<Expr> {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn try_binop(&mut self, lhand: Expr, op: Operand) -> Result<Expr> {
        error!().wrap()
    }

    pub fn try_operand(&mut self, op: Operand) -> Result<Expr> {
        if op == Operand::Plus || op == Operand::Minus {
            return self.try_operand_math(op);
        }

        error!().wrap()
    }

    pub fn try_operand_math(&mut self, op: Operand) -> Result<Expr> {
        let next = self.try_value_until(None)?;
        match next {
            Expr::Value(ref v) => {
                if let Value::String(_) = v {
                    return error!("cannout apply a mathematical expression to a string!").wrap();
                }

                if op == Operand::Minus {
                    return Ok(Expr::Not(Box::new(Expr::Value(v.clone()))));
                } else {
                    return Ok(next);
                }
            }
            _ => unreachable!(),
        };
    }
}
