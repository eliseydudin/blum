pub mod ast;
use ast::Parser;

fn main() {
    let buff = std::fs::read_to_string("test.blum").unwrap();
    let mut parser = Parser::new(buff);
    let errors = parser.parse();

    for error in errors {
        println!("{error}");
    }

    println!("{:#?}", parser.ast)
}
