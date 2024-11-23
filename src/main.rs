pub mod ast;

use ast::token::Token;

fn main() {
    let tokens = Token::parse_string("20 + 30 * (a + b)");
    println!("{:#?}", tokens)
}
