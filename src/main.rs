pub mod ast;
pub mod error;

fn main() {
    let source = "абвабвабвабвабвабвабвабвабвабвабвабвабвабвабвабвабвабвабвабвабв";
    let tokens = ast::TokenStream::new(source);
    let tokens = tokens.lex();

    println!("{tokens:#?}");
}
