use crate::dfa::DFA;
use std::collections::{HashSet, VecDeque};

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

    fn find_transitions(&self, state: i32, character: Option<u8>) -> Result<HashSet<i32>, String> {
        (self.transitions)(state, character)
    }

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
