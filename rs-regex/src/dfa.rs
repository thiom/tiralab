use std::collections::HashSet;

pub struct DFA {
    start_state: HashSet<i32>,
    accept_states: HashSet<i32>,
    transitions: Box<dyn Fn(HashSet<i32>, u8) -> HashSet<i32>>,
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

    fn is_accept_state(&self) -> bool {
        !(&self.dfa.accept_states & &self.current_state).is_empty()
    }

    pub fn accepts(&mut self, input: &[u8]) -> bool {
        for &symbol in input {
            self.make_transition(symbol);
        }
        self.is_accept_state()
    }
}
