use crate::nfa::NFA;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct NFAFragment {
    pub start_state: i32,
    pub accept_states: HashSet<i32>,
    transitions: HashMap<(i32, Option<u8>), HashSet<i32>>,
}

// An intermediate stage for processing the AST
impl NFAFragment {
    pub fn new(start_state: i32, accept_states: HashSet<i32>) -> Self {
        NFAFragment {
            start_state,
            accept_states,
            transitions: HashMap::new(),
        }
    }

    // Creates a transitions for a given symbol between states: from -> to
    pub fn create_transition(&mut self, from: i32, character: Option<u8>, to: i32) {
        match self.transitions.get_mut(&(from, character)) {
            Some(to_states) => {
                to_states.insert(to);
            }
            None => {
                let mut to_states = HashSet::<i32>::new();
                to_states.insert(to);
                self.transitions.insert((from, character), to_states);
            }
        }
    }

    // A skeleton for a new fragment with the context of previous transitions
    pub fn create_skeleton(&self) -> Self {
        NFAFragment {
            start_state: 0,
            accept_states: HashSet::new(),
            transitions: self.transitions.clone(),
        }
    }

    // '|' operator in the regex
    pub fn union_operator(&self, fragment: &NFAFragment) -> Self {
        let mut new_frag = self.create_skeleton();
        for (key, to_states) in fragment.create_skeleton().transitions {
            /*
            if !new_frag.transitions.contains_key(&key) {
                new_frag.transitions.insert(key, to_states);
            }
            */
            new_frag.transitions.entry(key).or_insert(to_states);
        }
        new_frag
    }

    // Converts the fragment into NFA
    pub fn to_nfa(self) -> NFA {
        let accepts_copy = self.accept_states.clone();
        let start_copy = self.start_state;
        let t = move |start: i32, character: Option<u8>| match self
            .transitions
            .get(&(start, character))
        {
            None => Err("Can't make transitions".to_string()),
            Some(to_states) => Ok(to_states.clone()),
        };
        NFA::new(start_copy, accepts_copy, Box::new(t))
    }
}
