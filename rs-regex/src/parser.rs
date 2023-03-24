use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType, Value};

pub struct Parser {
    scanner: Scanner,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut scanner: Scanner) -> Self {
        let current_token = Some(scanner.get_next_token());
        Parser {
            scanner,
            current_token,
        }
    }

    pub fn parse(&mut self) {
        self.print_tokens();
    }

    fn print_tokens(&mut self) {
        let tokens = self.get_all_tokens();
        for token in tokens {
            println!("{}", token);
        }
    }

    fn get_all_tokens(&mut self) -> Vec<Token> {
        let mut results = vec![self.current_token.as_ref().unwrap().clone()];
        self.current_token = Some(self.scanner.get_next_token());
        while let token = self.current_token.as_ref().unwrap().clone() {
            if let TokenType::EOF = token.type_ {
                return results;
            }
            results.push(token);
            self.current_token = Some(self.scanner.get_next_token());
        }
        results
    }
}
