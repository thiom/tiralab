use crate::dfa::DFA;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Regex {
    dfa: DFA,
}

impl Regex {
    pub fn new(regex: String) -> Result<Regex, String> {
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr()?;
        Ok(Regex { dfa: nfa.to_dfa() })
    }

    pub fn matches(&self, string: String) -> bool {
        let mut recognizer = self.dfa.recognizer();
        recognizer.accepts(string.as_bytes())
    }
}
