use ast::{lexer::Lexer, parser::Parser};
use std::{env::args, num::NonZero};

pub mod ast;

fn main() {
    let path = args().nth(1);

    match path {
        Some(path) => {
            let file_contents = std::fs::read_to_string(path.clone())
                .inspect_err(|e| {
                    crate::error(
                        NonZero::new(1).unwrap(),
                        format!("Error opening the file at `{path}`, error: {e}"),
                    )
                })
                .unwrap();

            let mut lexer = Lexer::new(file_contents);
            let mut parser = Parser::new(lexer.scan_tokens());
            let ast = parser.parse();

            println!("{ast:#?}")
        }
        None => crate::error(NonZero::new(1).unwrap(), "No source file given"),
    }
}

fn error(pos: NonZero<usize>, message: impl Into<String>) {
    let message: String = message.into();
    println!("[line {pos}] error: {message}")
}

fn error_at_token(token: &ast::Token, message: impl Into<String>) {
    let message: String = message.into();
    let pos = token.line;
    println!("[line {pos}] error: {message}")
}
