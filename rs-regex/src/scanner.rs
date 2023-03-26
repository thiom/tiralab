use crate::tokens::{Token, TokenType, Value};

pub struct Scanner {
    regex: String,
    pos: usize,
    current_char: Option<char>,
}

impl Scanner {
    pub fn new(regex: String) -> Self {
        Scanner {
            regex: regex.clone(),
            pos: 0,
            current_char: Some(regex.as_bytes()[0] as char),
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            match c {
                '*' => {
                    self.advance();
                    return Token::new(TokenType::Star, Value::Char(c));
                }
                '+' => {
                    self.advance();
                    return Token::new(TokenType::Plus, Value::Char(c));
                }
                '(' => {
                    self.advance();
                    return Token::new(TokenType::LeftParen, Value::Char(c));
                }
                ')' => {
                    self.advance();
                    return Token::new(TokenType::RightParen, Value::Char(c));
                }
                '|' => {
                    self.advance();
                    return Token::new(TokenType::Or, Value::Char(c));
                }
                _ => {
                    return self.str_to_match();
                }
            }
        }
        Token::new(TokenType::EOF, Value::None)
    }

    fn error(&self) {
        panic!("invalid character used in the regex");
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.regex.len() - 1 {
            self.current_char = None;
        } else {
            self.current_char = Some(self.regex.as_bytes()[self.pos] as char);
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos > self.regex.len() {
            None
        } else {
            Some(self.regex.as_bytes()[self.pos + 1] as char)
        }
    }

    fn str_to_match(&mut self) -> Token {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            match c {
                '.' => {
                    result.push(c);
                    self.advance();
                }
                _ => {
                    if c.is_alphanumeric() {
                        result.push(c);
                        self.advance();
                    } else {
                        return Token::new(TokenType::Str, Value::String(result.clone()));
                    }
                }
            }
        }
        if !result.is_empty() {
            return Token::new(TokenType::Str, Value::String(result.clone()));
        }
        Token::new(TokenType::EOF, Value::None)
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use crate::tokens::{Token, TokenType, Value};

    #[test]
    fn scanner_string_token() {
        let regex = "a.b";
        let token = Token::new(TokenType::Str, Value::String("a.b".to_string()));
        let mut scanner = Scanner::new(regex.to_string());
        let result = scanner.get_next_token();
        assert_eq!(result, token);
    }

    #[test]
    fn scanner_closure_tokens() {
        let regex = "a*b+";
        let token_a = Token::new(TokenType::Str, Value::String("a".to_string()));
        let token_star = Token::new(TokenType::Star, Value::Char("*".as_bytes()[0] as char));
        let token_b = Token::new(TokenType::Str, Value::String("b".to_string()));
        let token_plus = Token::new(TokenType::Plus, Value::Char("+".as_bytes()[0] as char));
        let mut scanner = Scanner::new(regex.to_string());

        let result = scanner.get_next_token();
        assert_eq!(result, token_a);

        let result = scanner.get_next_token();
        assert_eq!(result, token_star);

        let result = scanner.get_next_token();
        assert_eq!(result, token_b);

        let result = scanner.get_next_token();
        assert_eq!(result, token_plus);
    }

    #[test]
    fn scanner_group_and_or_tokens() {
        let regex = "(a|b)";
        let token_lparen = Token::new(TokenType::LeftParen, Value::Char("(".as_bytes()[0] as char));
        let token_a = Token::new(TokenType::Str, Value::String("a".to_string()));
        let token_or = Token::new(TokenType::Or, Value::Char("|".as_bytes()[0] as char));
        let token_b = Token::new(TokenType::Str, Value::String("b".to_string()));
        let token_rparen = Token::new(
            TokenType::RightParen,
            Value::Char(")".as_bytes()[0] as char),
        );
        let mut scanner = Scanner::new(regex.to_string());

        let result = scanner.get_next_token();
        assert_eq!(result, token_lparen);

        let result = scanner.get_next_token();
        assert_eq!(result, token_a);

        let result = scanner.get_next_token();
        assert_eq!(result, token_or);

        let result = scanner.get_next_token();
        assert_eq!(result, token_b);

        let result = scanner.get_next_token();
        assert_eq!(result, token_rparen);
    }
}
