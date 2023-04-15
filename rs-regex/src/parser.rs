use crate::ast::{Counter, Node};
use crate::nfa::NFA;
use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    scanner: Scanner,
    current_token: Token,
}

// Parses the regular expression by asking tokens from the scanner one by one
// and constructs and abstract syntax tree (AST). Also converts the AST into
// NFA at the end and returns it.
//
// Corresponds to the following context-free grammar (CFG)
//
// <expr>    ->  <subexpr> EOF
// <subexpr> ->  <seq> '|' <subexpr> | <seq>
// <seq>     ->  <subseq> | ''
// <subseq>  ->  <star> <subseq> | <star>
// <star>    ->  <factor> '*' | <factor>
// <factor>  ->  '(' <subexpr> ')' | ASCII_CHAR

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

    // Consumes a token and proceeds to the next one
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

    // Corresponds to the production:
    // <factor> -> '(' <subexpr> ')' | ASCII_CHAR
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

    // Corresponds to the production:
    // <star> -> <factor> '*' | <factor>
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

    // Corresponds to the production:
    // <seq> -> <subseq> | ''
    fn seq(&mut self) -> Result<Node, String> {
        match self.current_token.type_ {
            TokenType::LeftParen => self.subseq(),
            TokenType::Char => self.subseq(),
            _ => Ok(Node::character(0x00)),
        }
    }

    // Corresponds to the production:
    // <subseq> -><star> <subseq> | <star>
    fn subseq(&mut self) -> Result<Node, String> {
        let node = self.star()?;
        match self.current_token.type_ {
            TokenType::LeftParen => Ok(Node::concat(node, self.subseq()?)),
            TokenType::Char => Ok(Node::concat(node, self.subseq()?)),
            _ => Ok(node),
        }
    }

    // Corresponds to the production:
    // <expr> -> <subexpr> EOF
    pub fn expr(&mut self) -> Result<NFA, String> {
        let node = self.subexpr()?;
        self.eat(TokenType::EOF)?;

        let mut counter = Counter::new();
        let fragment = node.to_fragment(&mut counter);
        Ok(fragment.to_nfa())
    }

    // Corresponds to the producion:
    // <subexpr> -> <seq> '|' <subexpr> | <seq>
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

//grcov-excl-start

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Node;
    use crate::scanner::Scanner;
    use crate::tokens::TokenType;

    #[test]
    fn expr() {
        let regex = "(b|a)*".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr();
        assert!(nfa.is_ok());
    }

    #[test]
    fn subexpr() {
        let regex = "b|c".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        //parser.eat(TokenType::Union);
        let node = parser.subexpr().unwrap();
        match node {
            Node::Union { left: _, right: _ } => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn subseq() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let node = parser.subseq().unwrap();
        match node {
            Node::Concat { left: _, right: _ } => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn seq() {
        let regex = "a".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let node = parser.seq().unwrap();
        match node {
            Node::Character { character: _ } => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn star() {
        let regex = "c*".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let node = parser.star().unwrap();
        let mut correct = false;
        match node {
            Node::Star { operand: _ } => correct = true,
            _ => assert!(false),
        }
        assert!(correct);
    }

    #[test]
    fn factor() {
        let regex = "a|b".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let node = parser.factor().unwrap();
        match node {
            Node::Character { character: _ } => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn print_tokens() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        parser.print_tokens();
        assert!(true);
    }

    #[test]
    fn eat_and_advance() {
        let regex = "a|b".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);

        assert_eq!(parser.current_token.type_, TokenType::Char);
        parser.advance();
        assert_eq!(parser.current_token.type_, TokenType::Union);
        assert!(parser.eat(TokenType::Union).is_ok());
        assert_eq!(parser.current_token.type_, TokenType::Char);
    }
}
//grcov-excl-stop
