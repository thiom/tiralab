use crate::nfa_fragment::NFAFragment;

#[derive(Debug)]
pub enum Node {
    Character { character: u8 },
    Union { left: Box<Node>, right: Box<Node> },
    Concat { left: Box<Node>, right: Box<Node> },
    Star { operand: Box<Node> },
}

/// Abstract syntax tree (AST) nodes
impl Node {
    pub fn character(character: u8) -> Self {
        Node::Character { character }
    }

    /// '|' operator in regex
    pub fn union(left: Node, right: Node) -> Self {
        Node::Union {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// contatenation in regex, such as 'aa'
    pub fn concat(left: Node, right: Node) -> Self {
        Node::Concat {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// '*' operator in regex
    pub fn star(operand: Node) -> Self {
        Node::Star {
            operand: Box::new(operand),
        }
    }

    /// Recursively converts the AST into NFA fragments
    pub fn to_fragment(self, counter: &mut Counter) -> NFAFragment {
        match self {
            Node::Character { character } => {
                let lhs = counter.new_state();
                let rhs = counter.new_state();
                let mut fragment = NFAFragment::new(lhs, vec![rhs].into_iter().collect());
                fragment.create_transition(lhs, Some(character), rhs);
                fragment
            }

            Node::Union { left, right } => {
                let lhs = left.to_fragment(counter);
                let rhs = right.to_fragment(counter);
                let mut fragment = lhs.union_operator(&rhs);

                let start = counter.new_state();
                fragment.create_transition(start, None, lhs.start_state);
                fragment.create_transition(start, None, rhs.start_state);

                fragment.start_state = start;
                fragment.accept_states = &lhs.accept_states | &rhs.accept_states;
                fragment
            }

            Node::Concat { left, right } => {
                let lhs = left.to_fragment(counter);
                let rhs = right.to_fragment(counter);
                let mut fragment = lhs.union_operator(&rhs);

                for state in lhs.accept_states {
                    fragment.create_transition(state, None, rhs.start_state);
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
                    fragment.create_transition(*state, None, lhs.start_state);
                }

                fragment.create_transition(start, None, lhs.start_state);
                fragment.accept_states = &lhs.accept_states | &vec![start].into_iter().collect();
                fragment
            }
        }
    }
}

/// Used for labeling the states
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
