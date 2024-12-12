pub mod ast;
pub mod error;

fn main() {
    let source = "\"ababab\"";
    let tokens = ast::TokenStream::new(source);
    let tokens = tokens.lex();

    println!("{tokens:#?}");
}
