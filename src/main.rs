pub mod ast;

fn main() {
    let tokens = ast::Token::parse_string("10 + 20 + 30");
    println!("{:#?}", tokens)
}
