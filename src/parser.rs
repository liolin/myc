use crate::ast::{Expression, FunctionDefinition, Program, Return, Statement};
use crate::lexer;
use crate::Token;

pub fn parse(token_stream: impl Iterator<Item = lexer::Result<Token>>) -> Program {
    let mut parser = Parser {
        token_stream,
        current: None,
        errors: vec![],
    };

    let program = parser.parse_program();
    if !parser.is_empty() {
        panic!("garbage")
    }
    program
}

struct Parser<T: Iterator<Item = lexer::Result<Token>>> {
    token_stream: T,
    current: Option<Token>,
    errors: Vec<String>,
}

macro_rules! expect_token {
    ($a:expr, $b:expr, $c:expr) => {
        if $a != $b {
            panic!("Found {:?}, expected {:?}", $b, $a);
        }
        $c
    };
}

impl<T: Iterator<Item = lexer::Result<Token>>> Parser<T> {
    fn parse_program(&mut self) -> Program {
        self.bump();
        let function_definition = self.parse_function_definition();
        Program {
            function_definition,
        }
    }

    fn parse_function_definition(&mut self) -> FunctionDefinition {
        expect_token!(
            &lexer::Token::Int,
            self.current.as_ref().unwrap(),
            self.bump()
        );
        let Some(Token::Identifier(name)) = self.current.as_ref() else {
            panic!("Identifier was not found");
        };
        let name = name.clone();
        self.bump();

        expect_token!(
            &lexer::Token::OpenParenthesis,
            self.current.as_ref().unwrap(),
            self.bump()
        );
        expect_token!(
            &lexer::Token::Void,
            self.current.as_ref().unwrap(),
            self.bump()
        );

        expect_token!(
            &lexer::Token::CloseParenthesis,
            self.current.as_ref().unwrap(),
            self.bump()
        );
        expect_token!(
            &lexer::Token::OpenBrace,
            self.current.as_ref().unwrap(),
            self.bump()
        );
        let body = self.parse_statement();
        expect_token!(
            &lexer::Token::CloseBrace,
            self.current.as_ref().unwrap(),
            self.bump()
        );

        FunctionDefinition { name, body }
    }

    fn parse_statement(&mut self) -> Statement {
        expect_token!(
            &lexer::Token::Return,
            self.current.as_ref().unwrap(),
            self.bump()
        );
        let expression = self.parse_expression();
        expect_token!(
            &lexer::Token::Semicolon,
            self.current.as_ref().unwrap(),
            self.bump()
        );

        Statement::Return(Return { expression })
    }

    fn parse_expression(&mut self) -> Expression {
        let Some(Token::Constant(n)) = self.current.as_ref() else {
            panic!("error");
        };
        let n = n.clone();
        self.bump();
        Expression::Constant(n)
    }

    fn bump(&mut self) {
        self.current = self
            .token_stream
            .next()
            .map(|t| t.expect("Unexpected token"));
    }

    fn is_empty(&self) -> bool {
        self.current.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_applcation() {
        let token_stream = vec![
            Token::Int,
            Token::Identifier("main".into()),
            Token::OpenParenthesis,
            Token::Void,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Return,
            Token::Constant(2),
            Token::Semicolon,
            Token::CloseBrace,
        ]
        .into_iter()
        .map(|t| Ok(t));
        parse(token_stream);
    }

    #[test]
    fn parse_gargabe_at_the_end() {
        let token_stream = vec![
            Token::Int,
            Token::Identifier("main".into()),
            Token::OpenParenthesis,
            Token::Void,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Return,
            Token::Constant(2),
            Token::Semicolon,
            Token::CloseBrace,
            Token::Identifier("foo".into()),
        ]
        .into_iter()
        .map(|t| Ok(t));
        parse(token_stream);
    }
}
