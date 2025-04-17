use std::{error::Error, fmt::Display, str::Chars};

pub fn lex(source: &str) -> impl Iterator<Item = Result<Token>> {
    let mut chars = Cursor::new(source.chars());

    std::iter::from_fn(move || chars.lex())
}

struct Cursor<'a> {
    chars: Chars<'a>,
    current: char,
}

const EOF: char = '\0';

impl<'a> Cursor<'a> {
    fn new(mut chars: Chars<'a>) -> Self {
        let current = chars.next().unwrap_or(EOF);
        Self { chars, current }
    }

    fn lex(&mut self) -> Option<Result<Token>> {
        self.skip_whitespace();
        let token = match self.current {
            '(' => {
                self.bump();
                Token::OpenParenthesis
            }
            ')' => {
                self.bump();
                Token::CloseParenthesis
            }
            '{' => {
                self.bump();
                Token::OpenBrace
            }
            '}' => {
                self.bump();
                Token::CloseBrace
            }
            ';' => {
                self.bump();
                Token::Semicolon
            }
            '0'..='9' => {
                let r = self.constant();
                if r.is_err() {
                    return Some(r);
                }
                r.expect("Already checked")
            }
            'a'..='z' => self.identifier(),
            'A'..='Z' => self.identifier(),
            EOF => return None,
            _ => {
                let current = self.current;
                self.bump();
                return Some(Err(LexError::InvalidCharacter(current)));
            }
        };
        Some(Ok(token))
    }

    fn identifier(&mut self) -> Token {
        let mut buffer = String::new();
        buffer.push(self.current);
        while self.bump().is_alphanumeric() {
            buffer.push(self.current);
        }
        identifier_to_token(buffer)
    }

    fn constant(&mut self) -> Result<Token> {
        let mut buffer = String::new();
        buffer.push(self.current);
        while self.bump().is_alphanumeric() {
            buffer.push(self.current);
        }
        let n = buffer
            .parse()
            .map_err(|_| LexError::InvalidNumber(buffer))?;
        Ok(Token::Constant(n))
    }

    fn skip_whitespace(&mut self) -> char {
        while self.current.is_whitespace() {
            self.bump();
        }
        self.current
    }
    fn bump(&mut self) -> char {
        self.current = self.chars.next().unwrap_or(EOF);
        self.current
    }
}

fn identifier_to_token(identifier: String) -> Token {
    match identifier.as_str() {
        "int" => Token::Int,
        "void" => Token::Void,
        "return" => Token::Return,
        _ => Token::Identifier(identifier),
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(i32),

    // Keywords
    Int,
    Void,
    Return,

    // Things
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

pub fn identifier(s: impl Into<String>) -> Token {
    Token::Identifier(s.into())
}

pub fn constant(i: i32) -> Token {
    Token::Constant(i)
}

pub type Result<T> = std::result::Result<T, LexError>;

#[derive(Debug)]
pub enum LexError {
    InvalidNumber(String),
    InvalidCharacter(char),
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            LexError::InvalidNumber(s) => {
                format!("found an invalid token {s}. Expected a number token")
            }
            LexError::InvalidCharacter(c) => format!("found an invalid character {c}"),
        };

        write!(f, "{out}")
    }
}
impl Error for LexError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_to_token_int() {
        assert_eq!(Token::Int, identifier_to_token("int".into()));
    }

    #[test]
    fn identifier_to_token_void() {
        assert_eq!(Token::Void, identifier_to_token("void".into()));
    }

    #[test]
    fn identifier_to_token_return() {
        assert_eq!(Token::Return, identifier_to_token("return".into()));
    }

    #[test]
    fn identifier_to_token_asdf() {
        assert_eq!(
            Token::Identifier("asdf".into()),
            identifier_to_token("asdf".into())
        );
    }

    #[test]
    fn lex_identifier_asdf() {
        let source = "asdf";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::Identifier("asdf".into()), token);
    }

    #[test]
    fn lex_constant_1() {
        let source = "1";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::Constant(1), token);
    }

    #[test]
    fn lex_constant_10() {
        let source = "10";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::Constant(10), token);
    }

    #[test]
    fn lex_invalid_identifier() {
        let source = "1anInvalidIdentifier";
        let token = lex(source.into()).next().unwrap();
        assert!(token.is_err());
    }

    #[test]
    fn lex_int_keyword() {
        let source = "int";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::Int, token);
    }

    #[test]
    fn lex_void_keyword() {
        let source = "void";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::Void, token);
    }

    #[test]
    fn lex_return_keyword() {
        let source = "return";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::Return, token);
    }

    #[test]
    fn lex_open_parenthesis() {
        let source = "(";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::OpenParenthesis, token);
    }

    #[test]
    fn lex_close_parenthesis() {
        let source = ")";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::CloseParenthesis, token);
    }

    #[test]
    fn lex_open_brace() {
        let source = "{";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::OpenBrace, token);
    }

    #[test]
    fn lex_close_brace() {
        let source = "}";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::CloseBrace, token);
    }

    #[test]
    fn lex_semicolon() {
        let source = ";";
        let token = lex(source.into()).next().unwrap().unwrap();
        assert_eq!(Token::Semicolon, token);
    }

    #[test]
    fn lex_simple_applcation() {
        let source = "int main(void){return 2;}";
        let tokens = lex(source.into()).map(|t| t.unwrap()).collect::<Vec<_>>();
        assert_eq!(
            vec![
                Token::Int,
                Token::Identifier("main".into()),
                Token::OpenParenthesis,
                Token::Void,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Return,
                Token::Constant(2),
                Token::Semicolon,
                Token::CloseBrace
            ],
            tokens
        );
    }

    #[test]
    fn lex_blub() {
        let source = "int main    (   void)   {   return  0   ;   }";
        let lexed_successfully = lex(source.into()).all(|r| r.is_ok());
        assert!(lexed_successfully);
    }

    #[test]
    fn lex_catch_invalid_identifier() {
        let source = "@";
        let token = lex(source.into()).next().unwrap();
        assert!(token.is_err())
    }
}
