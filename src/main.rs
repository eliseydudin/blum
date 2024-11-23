pub mod ast;
use ast::Lexer;

fn main() {
    let source = "a.c%b*(10+20)".to_owned();
    let mut lexer = Lexer::new(source.chars());
    lexer.parse();
    let tokens = lexer.finish();

    println!("{:#?}", tokens)
}
