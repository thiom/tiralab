use crate::nfa_fragment::NFAFragment;

#[derive(Debug)]
pub enum Node {
    Character { character: u8 },
    Union { left: Box<Node>, right: Box<Node> },
    Concat { left: Box<Node>, right: Box<Node> },
    Star { operand: Box<Node> },
}

impl Node {
    pub fn character(character: u8) -> Self {
        Node::Character { character }
    }

    pub fn union(left: Node, right: Node) -> Self {
        Node::Union {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn concat(left: Node, right: Node) -> Self {
        Node::Concat {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn star(operand: Node) -> Self {
        Node::Star {
            operand: Box::new(operand),
        }
    }

    pub fn to_fragment(self, counter: &mut Counter) -> NFAFragment {
        match self {
            Node::Character { character } => {
                let lhs = counter.new_state();
                let rhs = counter.new_state();
                let mut fragment = NFAFragment::new(lhs, vec![rhs].into_iter().collect());
                fragment.connect(lhs, Some(character), rhs);
                fragment
            }

            Node::Union { left, right } => {
                let lhs = left.to_fragment(counter);
                let rhs = right.to_fragment(counter);
                let mut fragment = lhs.bin_op(&rhs);

                let start = counter.new_state();
                fragment.connect(start, None, lhs.start_state);
                fragment.connect(start, None, rhs.start_state);

                fragment.start_state = start;
                fragment.accept_states = &lhs.accept_states | &rhs.accept_states;
                fragment
            }

            Node::Concat { left, right } => {
                let lhs = left.to_fragment(counter);
                let rhs = right.to_fragment(counter);
                let mut fragment = lhs.bin_op(&rhs);

                for state in lhs.accept_states {
                    fragment.connect(state, None, rhs.start_state);
                }

                fragment.start_state = lhs.start_state;
                fragment.accept_states = rhs.accept_states;
                fragment
            }

            Node::Star { operand } => {
                let lhs = operand.to_fragment(counter);
                let mut fragment = lhs.create_skeleton();
                let start = counter.new_state();
                fragment.start_state = start;

                for state in lhs.accept_states.iter() {
                    fragment.connect(*state, None, lhs.start_state);
                }

                fragment.connect(start, None, lhs.start_state);
                fragment.accept_states = &lhs.accept_states | &vec![start].into_iter().collect();
                fragment
            }
        }
    }
}

pub struct Counter {
    state_count: i32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { state_count: 1 }
    }

    pub fn new_state(&mut self) -> i32 {
        self.state_count += 1;
        self.state_count
    }
}
