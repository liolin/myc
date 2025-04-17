pub mod lexer;

pub use lexer::*;

pub fn lex(source: &str) -> impl Iterator<Item = lexer::Result<Token>> {
    lexer::lex(source)
}

pub fn compile(source: &str) {}
