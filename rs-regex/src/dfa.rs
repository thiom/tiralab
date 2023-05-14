use std::collections::HashSet;

/// Deterministic finite automaton
/// https://en.wikipedia.org/wiki/Deterministic_finite_automaton
pub struct DFA {
    pub start_state: HashSet<i32>,
    pub accept_states: HashSet<i32>,
    pub transitions: Box<dyn Fn(HashSet<i32>, u8) -> HashSet<i32>>,
}

impl DFA {
    pub fn new(
        start_state: HashSet<i32>,
        accept_states: HashSet<i32>,
        transitions: Box<dyn Fn(HashSet<i32>, u8) -> HashSet<i32>>,
    ) -> Self {
        DFA {
            start_state,
            accept_states,
            transitions,
        }
    }

    pub fn recognizer(&self) -> Recognizer {
        Recognizer::new(self)
    }

    pub fn get_transition(&self, state: HashSet<i32>, character: u8) -> HashSet<i32> {
        (self.transitions)(state, character)
    }
}

/// The runtime for recognizing the input strings
pub struct Recognizer<'a> {
    dfa: &'a DFA,
    current_state: HashSet<i32>,
}

impl<'a> Recognizer<'a> {
    pub fn new(dfa: &'a DFA) -> Self {
        let state = dfa.start_state.clone();
        Recognizer {
            dfa,
            current_state: state,
        }
    }

    fn make_transition(&mut self, character: u8) {
        let state = self.current_state.clone();
        self.current_state = self.dfa.get_transition(state, character)
    }

    /// Check if the current state is an accept state
    fn is_accept_state(&self) -> bool {
        !(&self.dfa.accept_states & &self.current_state).is_empty()
    }

    /// Make transitions along the dfa given by the input string and check if
    /// We end up in an accept state
    pub fn accepts(&mut self, input: &[u8]) -> bool {
        for &symbol in input {
            self.make_transition(symbol);
        }
        self.is_accept_state()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    fn is_accept_state() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr();
        assert!(nfa.is_ok());
        let dfa = nfa.unwrap().to_dfa();
        let mut recognizer = dfa.recognizer();
        recognizer.make_transition('a' as u8);
        assert!(!recognizer.is_accept_state());
        recognizer.make_transition('b' as u8);
        assert!(recognizer.is_accept_state());
    }

    #[test]
    fn accepts() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr();
        assert!(nfa.is_ok());
        let dfa = nfa.unwrap().to_dfa();
        let mut recognizer = dfa.recognizer();
        let input = "ab".to_string().into_bytes();
        assert!(recognizer.accepts(&input));
    }

    #[test]
    fn rejects() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr();
        assert!(nfa.is_ok());
        let dfa = nfa.unwrap().to_dfa();
        let mut recognizer = dfa.recognizer();
        let input = "a".to_string().into_bytes();
        assert!(!recognizer.accepts(&input));
    }

    #[test]
    fn get_transition() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr();
        assert!(nfa.is_ok());
        let dfa = nfa.unwrap().to_dfa();
        let recognizer = dfa.recognizer();
        let state = dfa.get_transition(recognizer.current_state, 'a' as u8);
        assert!(state.contains(&mut 3));
    }
}
