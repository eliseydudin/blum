pub mod ast;
use ast::parser::Parser;

fn main() {
    let buff = std::fs::read_to_string("test.blum").unwrap();
    let mut parser = Parser::new(buff);
    parser.parse();

    for err in parser.errors {
        println!("error: {err}")
    }

    println!("{:?}", parser.ast);
}
