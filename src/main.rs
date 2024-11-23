pub mod ast;

fn main() {
    let tokens = ast::Token::parse_string("20 + 30 * (a + b)");
    println!("{:#?}", tokens)
}
