pub mod ast;
pub mod error;

fn main() {
    let source = "a";
    let tokens = ast::Lexer::new(source);
    let tokens = tokens.lex();

    println!("{tokens:#?}");
}
