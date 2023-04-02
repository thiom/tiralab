use crate::tokens::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
    regex: String,
}

impl Scanner {
    pub fn new(regex: String) -> Self {
        Scanner {
            regex: regex.chars().rev().collect(),
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        match self.regex.pop() {
            Some(c) => match c {
                '\\' => Token::new(TokenType::Char, self.regex.pop().unwrap() as u8),
                '|' => Token::new(TokenType::Union, c as u8),
                '*' => Token::new(TokenType::Star, c as u8),
                '(' => Token::new(TokenType::LeftParen, c as u8),
                ')' => Token::new(TokenType::RightParen, c as u8),
                _ => Token::new(TokenType::Char, c as u8),
            },
            None => Token::new(TokenType::EOF, 0xff),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use crate::tokens::{Token, TokenType};

    #[test]
    fn scanner_string_token() {
        let regex = "a";
        let token = Token::new(TokenType::Char, "a".as_bytes()[0] as u8);
        let mut scanner = Scanner::new(regex.to_string());
        let result = scanner.get_next_token();
        assert_eq!(result, token);
    }

    #[test]
    fn scanner_closure_tokens() {
        let regex = "a*b";
        let mut scanner = Scanner::new(regex.to_string());
        let token_a = Token::new(TokenType::Char, "a".as_bytes()[0] as u8);
        let token_star = Token::new(TokenType::Star, "*".as_bytes()[0] as u8);
        let token_b = Token::new(TokenType::Char, "b".as_bytes()[0] as u8);

        let result = scanner.get_next_token();
        assert_eq!(result, token_a);

        let result = scanner.get_next_token();
        assert_eq!(result, token_star);

        let result = scanner.get_next_token();
        assert_eq!(result, token_b);
    }

    #[test]
    fn scanner_group_and_union_tokens() {
        let regex = "a(b|a)";
        let mut scanner = Scanner::new(regex.to_string());
        let token_a = Token::new(TokenType::Char, "a".as_bytes()[0] as u8);
        let token_lparen = Token::new(TokenType::LeftParen, "(".as_bytes()[0] as u8);
        let token_b = Token::new(TokenType::Char, "b".as_bytes()[0] as u8);
        let token_union = Token::new(TokenType::Union, "|".as_bytes()[0] as u8);
        let token_rparen = Token::new(TokenType::RightParen, ")".as_bytes()[0] as u8);

        let result = scanner.get_next_token();
        assert_eq!(result, token_a);

        let result = scanner.get_next_token();
        assert_eq!(result, token_lparen);

        let result = scanner.get_next_token();
        assert_eq!(result, token_b);

        let result = scanner.get_next_token();
        assert_eq!(result, token_union);

        let result = scanner.get_next_token();
        assert_eq!(result, token_a);

        let result = scanner.get_next_token();
        assert_eq!(result, token_rparen);
    }
}
