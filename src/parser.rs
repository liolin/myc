use std::error::Error;
use std::fmt::Display;
use std::iter::Peekable;

use crate::ast::{Expression, FunctionDefinition, Program, Statement, UnaryOperation};
use crate::lexer;
use crate::Token;

pub fn parse(token_stream: impl Iterator<Item = Token>) -> Result<Program> {
    let mut parser = Parser {
        token_stream: token_stream.peekable(),
    };

    let program = parser.parse_program();
    if !parser.is_empty() {
        return Err(ParseError::UnexpectedToken(
            parser.bump().expect("should be checked by is_empty"),
        ));
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
        let t = self.bump().ok_or(ParseError::UnexpectedEOF)?;
        let Token::Identifier(name) = t else {
            return Err(ParseError::UnexpectedToken(t));
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
        Ok(Statement::Return(expression))
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        let t = self.bump().ok_or(ParseError::UnexpectedEOF)?;
        let exp = match t {
            Token::Constant(n) => Expression::Constant(n),
            Token::OpenParenthesis => {
                let exp = self.parse_expression()?;
                self.bump_if_equal(&lexer::Token::CloseParenthesis)?;
                exp
            }
            Token::Minus | Token::Tilde => self.parse_unary_operation(t)?,
            t @ _ => return Err(ParseError::UnexpectedToken(t.clone())),
        };
        Ok(exp)
    }

    fn parse_unary_operation(&mut self, token: Token) -> Result<Expression> {
        let op = match token {
            Token::Minus => UnaryOperation::Negate,
            Token::Tilde => UnaryOperation::Complement,
            t @ _ => return Err(ParseError::UnexpectedToken(t)),
        };
        let exp = self.parse_expression()?;
        Ok(Expression::Unary(op, Box::new(exp)))
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
            return Err(ParseError::UnexpectedToken(p.clone()));
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
    UnexpectedToken(Token),
    UnexpectedEOF,
    LexError,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::UnexpectedToken(t) => format!("found an unexpected token {t}"),
            Self::UnexpectedEOF => "reached unexpected EOF".into(),
            Self::LexError => "encountered an lexing error".into(),
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
                body: Statement::Return(Expression::Constant(2)),
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
