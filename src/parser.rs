use std::error::Error;
use std::fmt::Display;
use std::iter::Peekable;

use crate::ast::{Expression, FunctionDefinition, Program, Return, Statement};
use crate::lexer;
use crate::Token;

pub fn parse(token_stream: impl Iterator<Item = Token>) -> Result<Program> {
    let mut parser = Parser {
        token_stream: token_stream.peekable(),
    };

    let program = parser.parse_program();
    if !parser.is_empty() {
        return Err(ParseError::UnexpectedToken);
    }
    program
}

struct Parser<T: Iterator<Item = Token>> {
    token_stream: Peekable<T>,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    fn parse_program(&mut self) -> Result<Program> {
        Ok(Program {
            function_definition: self.parse_function_definition()?,
        })
    }

    fn parse_function_definition(&mut self) -> Result<FunctionDefinition> {
        self.bump_if_equal(&lexer::Token::Int)?;
        let Some(Token::Identifier(name)) = self.bump() else {
            return Err(ParseError::UnexpectedToken);
        };
        self.bump_if_equal(&lexer::Token::OpenParenthesis)?;
        self.bump_if_equal(&lexer::Token::Void)?;
        self.bump_if_equal(&lexer::Token::CloseParenthesis)?;
        self.bump_if_equal(&lexer::Token::OpenBrace)?;

        let body = self.parse_statement()?;

        self.bump_if_equal(&lexer::Token::CloseBrace)?;

        Ok(FunctionDefinition { name, body })
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        self.bump_if_equal(&lexer::Token::Return)?;

        let expression = self.parse_expression()?;

        self.bump_if_equal(&lexer::Token::Semicolon)?;
        Ok(Statement::Return(Return { expression }))
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        let Some(Token::Constant(n)) = self.bump() else {
            return Err(ParseError::UnexpectedToken);
        };
        let n = n.clone();
        Ok(Expression::Constant(n))
    }

    /// Checks if the `token_stream` is empty.
    /// Does not consume the token_stream.
    fn is_empty(&mut self) -> bool {
        self.token_stream.peek().is_none()
    }

    /// Advances the token stream and returns the next Token if any.
    fn bump(&mut self) -> Option<Token> {
        self.token_stream.next()
    }

    /// Returns Ok(()) if the next token is equal to `expected_token`.
    fn expect_token(&mut self, expected_token: &Token) -> Result<()> {
        let p = self.token_stream.peek().ok_or(ParseError::UnexpectedEOF)?;
        if p != expected_token {
            return Err(ParseError::UnexpectedToken);
        }
        Ok(())
    }

    /// Advances the token stream and returns the next Token if the current is equal to the `expected_token`.
    fn bump_if_equal(&mut self, expected_token: &Token) -> Result<Token> {
        self.expect_token(expected_token)?;
        Ok(self
            .bump()
            .expect("should be checked by `expect_token` and return early if None"))
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    UnexpectedEOF,
    LexError,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::UnexpectedToken => "found an unexpected token",
            Self::UnexpectedEOF => "reached unexpected EOF",
            Self::LexError => "encountered an lexing error",
        };
        write!(f, "{s}")
    }
}

impl Error for ParseError {}

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
        .into_iter();

        let expected_ast = Program {
            function_definition: FunctionDefinition {
                name: "main".into(),
                body: Statement::Return(Return {
                    expression: Expression::Constant(2),
                }),
            },
        };

        let ast = parse(token_stream).unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn invalid_function_definition_missing_open_parenthesis() {
        let token_stream = vec![
            Token::Int,
            Token::Identifier("main".into()),
            Token::Void,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Return,
            Token::Constant(2),
            Token::Semicolon,
            Token::CloseBrace,
        ]
        .into_iter();
        parse(token_stream).unwrap_err();
    }

    #[test]
    fn invalid_function_definition_missing_close_parenthesis() {
        let token_stream = vec![
            Token::Int,
            Token::Identifier("main".into()),
            Token::OpenParenthesis,
            Token::Void,
            Token::OpenBrace,
            Token::Return,
            Token::Constant(2),
            Token::Semicolon,
            Token::CloseBrace,
        ]
        .into_iter();
        parse(token_stream).unwrap_err();
    }

    #[test]
    fn invalid_function_definition_missing_open_brace() {
        let token_stream = vec![
            Token::Int,
            Token::Identifier("main".into()),
            Token::OpenParenthesis,
            Token::Void,
            Token::CloseParenthesis,
            Token::Return,
            Token::Constant(2),
            Token::Semicolon,
            Token::CloseBrace,
        ]
        .into_iter();
        parse(token_stream).unwrap_err();
    }

    #[test]
    fn invalid_function_definition_missing_close_brace() {
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
        ]
        .into_iter();
        parse(token_stream).unwrap_err();
    }

    #[test]
    fn invalid_gargabe_at_the_end() {
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
        .into_iter();
        parse(token_stream).unwrap_err();
    }
}
