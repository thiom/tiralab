mod parser;
mod scanner;
mod tokens;

use crate::parser::Parser;
use crate::scanner::Scanner;
use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    regex: String,
}

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
        .get_matches();

    Ok(Config {
        regex: matches.value_of_lossy("regex").unwrap().trim().to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("REGEX: {}", &config.regex);
    let scanner = Scanner::new(config.regex);
    let mut parser = Parser::new(scanner);
    parser.parse();
    Ok(())
}
