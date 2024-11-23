pub mod ast;

fn main() {
    let tokens = ast::Token::parse_string("aab+20 + 30");
    println!("{:#?}", tokens)
}
