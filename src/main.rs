pub mod ast;
pub mod error;

fn main() {
    let source = "\"abab\"\nабабаб";
    let tokens = ast::Lexer::new(source);
    let tokens = tokens.lex();

    println!("{tokens:#?}");
}
