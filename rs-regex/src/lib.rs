mod ast;
mod dfa;
mod nfa;
mod nfa_fragment;
mod parser;
mod regex;
mod scanner;
mod tokens;

use crate::parser::Parser;
use crate::regex::Regex;
use crate::scanner::Scanner;
use clap::{App, Arg};
use std::error::Error;
use std::io::{stdin, stdout, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    regex: String,
    print_tokens: bool,
}

// Processes the cli arguments
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("rs_regex")
        .version("0.1.0")
        .author("thiom")
        .about("Rust regex engine")
        .arg(
            Arg::with_name("regex")
                .value_name("REGEX")
                .required(true)
                .help("Regular expession"),
        )
        .arg(
            Arg::with_name("print_tokens")
                .takes_value(false)
                .short("t")
                .long("print_tokens")
                .help("prints tokens"),
        )
        .get_matches();

    Ok(Config {
        regex: matches.value_of_lossy("regex").unwrap().trim().to_string(),
        print_tokens: matches.is_present("print_tokens"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    if config.print_tokens {
        let scanner = Scanner::new(config.regex);
        let mut parser = Parser::new(scanner);
        parser.print_tokens();
    } else {
        if !config.regex.to_string().is_ascii() {
            println!("Regular expression must contain valid ASCII characters only. Try again");
            return Ok(());
        };
        let regex = Regex::new(config.regex.to_string()).unwrap();
        loop {
            let mut input = String::new();
            println!("\nGive a string (empty string will exit)");
            println!("Regular expression is: {}", &config.regex);
            let _ = stdout().flush();
            stdin()
                .read_line(&mut input)
                .expect("Did not enter a correct string");
            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }
            if input.is_empty() {
                break;
            }
            if regex.matches(input.to_string()).unwrap() {
                println!("MATCH");
            } else {
                println!("NO MATCH");
            }
        }
    };
    Ok(())
}
