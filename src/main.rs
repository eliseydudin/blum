pub mod ast;

fn main() {
    let tokens = ast::Token::parse_string("абаба % 20 * (30 + 40)");
    println!("{:#?}", tokens)
}
