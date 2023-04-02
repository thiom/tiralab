use crate::ast::{Counter, Node};
use crate::nfa::NFA;
use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    scanner: Scanner,
    current_token: Token,
}

impl Parser {
    pub fn new(scanner: Scanner) -> Self {
        let mut parser = Parser {
            scanner,
            current_token: Token::new(TokenType::NoOp, 0x00),
        };
        parser.advance();
        parser
    }

    pub fn print_tokens(&mut self) {
        let mut tokens = vec![self.current_token.clone()];
        self.current_token = self.scanner.get_next_token();
        loop {
            let token = self.current_token.clone();
            if let TokenType::EOF = token.type_ {
                break;
            }
            tokens.push(token);
            self.current_token = self.scanner.get_next_token();
        }
        for token in tokens {
            println!("{}", token);
        }
    }

    pub fn eat(&mut self, token: TokenType) -> Result<(), String> {
        if self.current_token.type_ != token {
            return Err("parsing error: unexpected token".to_string());
        }
        self.advance();
        Ok(())
    }

    pub fn advance(&mut self) {
        self.current_token = self.scanner.get_next_token()
    }

    fn factor(&mut self) -> Result<Node, String> {
        match self.current_token.type_ {
            TokenType::LeftParen => {
                self.eat(TokenType::LeftParen)?;
                let node = self.subexpr()?;
                self.eat(TokenType::RightParen)?;
                Ok(node)
            }
            _ => {
                let node = Node::character(self.current_token.value);
                self.eat(TokenType::Char)?;
                Ok(node)
            }
        }
    }

    fn star(&mut self) -> Result<Node, String> {
        let node = self.factor()?;
        match self.current_token.type_ {
            TokenType::Star => {
                self.eat(TokenType::Star)?;
                Ok(Node::star(node))
            }
            _ => Ok(node),
        }
    }

    fn seq(&mut self) -> Result<Node, String> {
        match self.current_token.type_ {
            TokenType::LeftParen => self.subseq(),
            TokenType::Char => self.subseq(),
            _ => Ok(Node::character(0x00)),
        }
    }

    fn subseq(&mut self) -> Result<Node, String> {
        let node = self.star()?;
        match self.current_token.type_ {
            TokenType::LeftParen => Ok(Node::concat(node, self.subseq()?)),
            TokenType::Char => Ok(Node::concat(node, self.subseq()?)),
            _ => Ok(node),
        }
    }

    pub fn expr(&mut self) -> Result<NFA, String> {
        let node = self.subexpr()?;
        self.eat(TokenType::EOF)?;

        let mut counter = Counter::new();
        let fragment = node.to_fragment(&mut counter);
        Ok(fragment.to_nfa())
    }

    fn subexpr(&mut self) -> Result<Node, String> {
        let node = self.seq()?;
        match self.current_token.type_ {
            TokenType::Union => {
                self.eat(TokenType::Union)?;
                Ok(Node::union(node, self.subexpr()?))
            }
            _ => Ok(node),
        }
    }
}
