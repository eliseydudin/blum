use std::num::NonZero;

pub mod ast;

fn main() {}

fn error(pos: NonZero<usize>, message: impl Into<String>) {
    let message: String = message.into();
    println!("[line {pos}] error: {message}")
}

fn error_at_token(token: &ast::Token, message: impl Into<String>) {
    let message: String = message.into();
    let pos = token.line;
    println!("[line {pos}] error: {message}")
}
