mod scanner;
mod token;
use crate::token::Token;


use clap::{App, Arg, ArgMatches};
use std::fs::File;

use std::io::{stdin, stdout, BufReader};
use std::option::Option;

//Add Trait for BufReader to read_to_string
use std::io::prelude::{Read, Write};


fn error(line_num: i8, message: &str) {
    report(line_num, "", message);
}

fn report(line_num: i8, loc: &str, msg: &str) {
    eprintln!("[line {}] Error {}: {}", line_num, loc, msg);
}

/// Parse program arguments
/// The 'static means the ArgMatches lifetime will last the whole program
pub fn parse_args() -> ArgMatches<'static> {
    App::new("Lox interpreter")
        .version("0.1.0")
        .author("Paul Naranja")
        .about("A Lox interpreter")
        .arg(
            Arg::with_name("script")
                .help("Lox script name")
                .short("l")
                .takes_value(true),
        )
        .get_matches()
}

/// Interpret the source input
fn run(src: &str) {
    let a = Token::new(token::token_type::AND, String::from(""), None, 0);
    src.split_whitespace().for_each(|t| println!("{}", t));
}

/// Start a repl
fn run_prompt() {
    let mut buffer = String::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buffer).unwrap();
        run(&buffer);
        buffer = String::from("");
    }
}

/// Read the file and return the contents as a string
fn extract_file(src: &str) -> Option<String> {
    let file = File::open(src).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut src_contents = String::new();
    buf_reader.read_to_string(&mut src_contents).unwrap();
    Some(src_contents)
}

fn main() {
    let s = parse_args();
    let script = s.value_of("script").or_else(|| {
        run_prompt();
        Some("")
    });
    script
        .and_then(|s| extract_file(s))
        .map(|str| run(&str))
        .unwrap();
}
