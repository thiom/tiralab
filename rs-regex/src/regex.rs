use crate::dfa::DFA;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Regex {
    dfa: DFA,
}

impl Regex {
    pub fn new(regex: String) -> Result<Regex, String> {
        if !regex.is_ascii() {
            return Err("Regex was not ascii".to_string());
        }
        let scanner = Scanner::new(regex);
        let mut parser = Parser::new(scanner);
        let nfa = parser.expr()?;
        Ok(Regex { dfa: nfa.to_dfa() })
    }

    // Tries to recognize the input string against the DFA
    pub fn matches(&self, string: String) -> Result<bool, String> {
        if !string.is_ascii() {
            return Err("Input string was not ascii".to_string());
        }
        let mut recognizer = self.dfa.recognizer();
        Ok(recognizer.accepts(string.as_bytes()))
    }
}

//grcov-excl-start

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn fails() {
        let regex = "(asd";
        let dfa = Regex::new(regex.to_string());
        assert!(!dfa.is_ok());
    }

    #[test]
    fn fails2() {
        let regex = "¥";
        let dfa = Regex::new(regex.to_string());
        assert!(!dfa.is_ok());
    }

    #[test]
    fn fails3() {
        let regex = "asd";
        let dfa = Regex::new(regex.to_string()).unwrap();
        let m = dfa.matches("¥".to_string());
        assert!(!m.is_ok());
    }

    #[test]
    fn basic_lowercase() {
        let regex = "aa(b|cc)*a";
        let dfa = Regex::new(regex.to_string()).unwrap();
        let to_accept = vec!["aaa", "aaba", "aacca", "aabba", "aabbccbbbccbcca"];
        let to_deny = vec!["aa", "aabaa", "aaccca", "aabbac", "bbccbbbccbcca"];
        for s in to_accept {
            assert!(dfa.matches(s.to_string()).unwrap());
        }
        for s in to_deny {
            assert!(!dfa.matches(s.to_string()).unwrap());
        }
    }

    #[test]
    fn basic_upper_lower_case() {
        let regex = "(B|cc|GG)*A";
        let dfa = Regex::new(regex.to_string()).unwrap();
        let to_accept = vec!["BA", "A", "ccGGA", "BBBA", "BBGGBBccA", "ccccccGGBGGA"];
        let to_deny = vec!["B", "", "AA", "cccA", "AB", "BcA", "BB", "BBBBBBB", "F"];
        for s in to_accept {
            assert!(dfa.matches(s.to_string()).unwrap());
        }
        for s in to_deny {
            assert!(!dfa.matches(s.to_string()).unwrap());
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
            for _ in 0..s_len {
                let c = rng.gen_range(97..123) as u8 as char;
                s.push(c.to_owned());
            }
            to_accept.push(s);
            s_len = s_len + 100;
        }
        for string in to_accept {
            assert!(dfa.matches(string.to_string()).unwrap());
        }
    }

    #[test]
    fn random_lower_upper_lower() {
        let mut regex = "".to_string();
        let lower = "(a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)*".to_string();
        let upper = lower.to_uppercase();
        regex.push_str(&lower);
        regex.push_str(&upper);
        regex.push_str(&lower);
        let dfa = Regex::new(regex).unwrap();
        let mut to_accept = vec!["".to_string()];
        let mut rng = rand::thread_rng();
        let mut s_len = 10;
        while s_len < 500 {
            let mut s = String::from("");
            for _ in 0..s_len {
                let c = rng.gen_range(97..123) as u8 as char;
                s.push(c.to_owned());
            }
            for _ in 0..s_len {
                let c = rng.gen_range(65..91) as u8 as char;
                s.push(c.to_owned());
            }
            for _ in 0..s_len {
                let c = rng.gen_range(97..123) as u8 as char;
                s.push(c.to_owned());
            }
            to_accept.push(s);
            s_len = s_len + 100;
        }
        for string in to_accept {
            assert!(dfa.matches(string).unwrap());
        }
    }
}

//grcov-excl-stop
