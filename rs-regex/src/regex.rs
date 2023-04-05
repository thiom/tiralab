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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn basic_lowercase() {
        let regex = "aa(b|cc)*a";
        let dfa = Regex::new(regex.to_string()).unwrap();
        let to_accept = vec!["aaa", "aaba", "aacca", "aabba", "aabbccbbbccbcca"];
        let to_deny = vec!["aa", "aabaa", "aaccca", "aabbac", "bbccbbbccbcca"];
        for s in to_accept {
            assert!(dfa.matches(s.to_string()));
        }
        for s in to_deny {
            assert!(!dfa.matches(s.to_string()));
        }
    }

    #[test]
    fn basic_uppercase() {
        let regex = "(B|cc|GG)*A";
        let dfa = Regex::new(regex.to_string()).unwrap();
        let to_accept = vec!["BA", "A", "ccGGA", "BBBA", "BBGGBBccA", "ccccccGGBGGA"];
        let to_deny = vec!["B", "", "AA", "cccA", "AB", "BcA", "BB", "BBBBBBB", "F"];
        for s in to_accept {
            assert!(dfa.matches(s.to_string()));
        }
        for s in to_deny {
            assert!(!dfa.matches(s.to_string()));
        }
    }

    #[test]
    fn random_lowercase() {
        let regex = "(a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)*";
        let dfa = Regex::new(regex.to_string()).unwrap();
        let mut to_accept = vec!["".to_string()];
        let mut rng = rand::thread_rng();
        let mut s_len = 10;
        while s_len < 1000 {
            let mut s = String::from("");
            for i in 0..s_len {
                let c = rng.gen_range(97..123) as u8 as char;
                s.push(c.to_owned());
            }
            to_accept.push(s);
            s_len = s_len + 100;
        }
        for string in to_accept {
            assert!(dfa.matches(string.to_string()));
        }
    }
}
