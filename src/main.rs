pub mod ast;
use ast::Parser;

fn main() {
    let buff = std::fs::read_to_string("test.blum").unwrap();
    let parser = Parser::new(buff);
    println!("{:#?}", parser.tokens);
}
