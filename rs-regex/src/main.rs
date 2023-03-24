mod parser;
mod scanner;
mod tokens;

fn main() {
    if let Err(e) = rs_regex::get_args().and_then(rs_regex::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
