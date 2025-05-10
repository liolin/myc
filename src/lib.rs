pub mod assembly;
pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod tacky;

pub use lexer::*;

pub fn lex(source: &str) -> impl Iterator<Item = Token> {
    lexer::lex(source)
}

pub fn parse(
    token_stream: impl Iterator<Item = Token>,
) -> Result<ast::Program, parser::ParseError> {
    parser::parse(token_stream)
}

pub fn tacky(program: ast::Program) -> tacky::Program {
    tacky::tacky(program)
}

pub fn assembly(program: tacky::Program) -> assembly::Program {
    assembly::assembly(program)
}

pub fn codegen(program: assembly::Program) -> String {
    codegen::codegen(program)
}
