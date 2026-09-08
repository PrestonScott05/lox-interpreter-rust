use crate::token_type::TokenType;
use std::fmt;
//add traits
#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Str(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        // just getting used to the last thing in a functino being it's return value
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self,   f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}