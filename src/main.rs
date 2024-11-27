pub mod ast;
use ast::parser::Parser;

fn main() {
    let buff = std::fs::read_to_string("test.blum").unwrap();
    let mut parser = Parser::new(buff);
    println!("{:#?}", parser.tokens);
    parser.parse();

    for err in parser.errors {
        println!("error: {err}")
    }
    println!("\n\nAST BEGIN\n");

    println!("{:#?}", parser.ast);
}
