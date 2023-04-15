use crate::dfa::DFA;
use std::collections::{HashSet, VecDeque};

//Nondeterministic finite automaton
//https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton
pub struct NFA {
    start_state: i32,
    accept_states: HashSet<i32>,
    transitions: Box<dyn Fn(i32, Option<u8>) -> Result<HashSet<i32>, String>>,
}

impl NFA {
    pub fn new(
        start_state: i32,
        accept_states: HashSet<i32>,
        transitions: Box<dyn Fn(i32, Option<u8>) -> Result<HashSet<i32>, String>>,
    ) -> Self {
        NFA {
            start_state,
            accept_states,
            transitions,
        }
    }

    //finds the destination state from a given state and a symbol
    fn find_transitions(&self, state: i32, character: Option<u8>) -> Result<HashSet<i32>, String> {
        (self.transitions)(state, character)
    }

    //eliminates the epsilon transitions for the dfa conversion
    fn transform_transitions(&self, set: HashSet<i32>) -> HashSet<i32> {
        let mut not_visited: VecDeque<i32> = set.into_iter().collect();
        let mut visited = VecDeque::<i32>::new();
        while !not_visited.is_empty() {
            let start = not_visited.pop_back().unwrap();
            visited.push_back(start);
            match self.find_transitions(start, None) {
                Ok(states) => {
                    for state in states {
                        if !visited.contains(&state) {
                            not_visited.push_front(state)
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        visited.into_iter().collect()
    }

    //converts the nfa into a corresponding dfa
    pub fn to_dfa(self) -> DFA {
        let start =
            self.transform_transitions(vec![self.start_state.clone()].into_iter().collect());
        let accept = self.accept_states.clone();
        let transition_set = move |set: HashSet<i32>, character: u8| {
            let mut result = HashSet::<i32>::new();
            for state in set {
                match self.find_transitions(state, Some(character)) {
                    Ok(set) => result = &result | &set,
                    Err(_) => continue,
                }
            }
            self.transform_transitions(result)
        };
        DFA::new(start, accept, Box::new(transition_set))
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    //use crate::tokens::TokenType;
    //use rand::Rng;

    #[test]
    fn start_state() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr();
        assert!(nfa.is_ok());
        assert_eq!(nfa.unwrap().start_state, 2);
    }

    #[test]
    fn accept_state() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr();
        assert!(nfa.is_ok());
        assert!(nfa.unwrap().accept_states.contains(&mut 5));
    }

    #[test]
    fn to_dfa_start() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr();
        assert!(nfa.is_ok());
        let dfa = nfa.unwrap().to_dfa();
        assert!(dfa.start_state.contains(&mut 2));
    }

    #[test]
    fn to_dfa_acc() {
        let regex = "ab".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr();
        assert!(nfa.is_ok());
        let dfa = nfa.unwrap().to_dfa();
        assert!(dfa.accept_states.contains(&mut 5));
    }

    #[test]
    fn find_transitions() {
        let regex = "a(b|c)".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr().unwrap();
        let result = nfa.find_transitions(2, Some('a' as u8)).unwrap();
        assert!(result.contains(&mut 3));
    }

    #[test]
    fn trasform_epsilon() {
        let regex = "a|b".to_string();
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr().unwrap();
        let start_states =
            nfa.transform_transitions(vec![nfa.start_state.clone()].into_iter().collect());
        assert!(start_states.contains(&mut 4));
        assert!(start_states.contains(&mut 2));
    }
}
