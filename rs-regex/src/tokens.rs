use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, PartialOrd, PartialEq, Clone)]
pub enum TokenType {
    Star,
    RightParen,
    LeftParen,
    Char,
    Union,
    NoOp,
    EOF,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub type_: TokenType,
    pub value: u8,
}

impl Token {
    pub fn new(type_: TokenType, value: u8) -> Self {
        Token { type_, value }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Token({:?}, {})", self.type_, self.value as char)
    }
}
