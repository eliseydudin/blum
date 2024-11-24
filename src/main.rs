pub mod ast;
use ast::Lexer;

fn main() {
    let buff = std::fs::read_to_string("test.bl").unwrap();
    let mut lexer = Lexer::new(buff.chars());
    lexer.parse();
    let tokens = lexer.finish();

    println!("{:#?}", tokens)
}
