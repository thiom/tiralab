use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Star,
    Plus,
    RightParen,
    LeftParen,
    Str,
    Or,
    EOF,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Char(char),
    String(String),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Char(c) => write!(f, "{}", c),
            Value::String(s) => write!(f, "{}", s),
            Value::None => write!(f, ""),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub type_: TokenType,
    pub value: Value,
}

impl Token {
    pub fn new(type_: TokenType, value: Value) -> Self {
        Token { type_, value }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Token({:?}, {})", self.type_, self.value.to_string())
    }
}
