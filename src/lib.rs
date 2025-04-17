pub mod ast;
pub mod lexer;
pub mod parser;

pub use lexer::*;

pub fn lex(source: &str) -> impl Iterator<Item = lexer::Result<Token>> {
    lexer::lex(source)
}

pub fn parse(token_stream: impl Iterator<Item = lexer::Result<Token>>) -> ast::Program {
    parser::parse(token_stream)
}

pub fn compile(source: &str) {}
