extern crate regex;
use regex::Regex;
use std::env;
use std::io::{self, BufRead};
use std::process;

fn convert_to_regex_pattern(pattern: &str) -> String {
    match pattern {
        "\\d" => r"\d".to_string(),
        "\\w" => r"\w".to_string(),
        _ => pattern.to_string(),
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else if pattern == "\\d" {
        return input_line.contains(|c: char| c.is_digit(10));
    } else if pattern == "\\w" {
        return input_line.contains(|c: char| c.is_alphanumeric());
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        if pattern.starts_with("[^") {
            let new_pattern = &pattern[2..pattern.len() - 1];
            !input_line.chars().any(|c| new_pattern.contains(c))
        } else {
            let new_pattern = &pattern[1..pattern.len() - 1];
            input_line.chars().any(|c| new_pattern.contains(c))
        }
    } else {
        let regex_pattern = convert_to_regex_pattern(pattern);
        let re = Regex::new(&regex_pattern).unwrap();
        re.is_match(input_line)
    }
}

// usage- echo <input_text> | your_program.sh -E <pattern>

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
    } else {
        let pattern = env::args().nth(2).unwrap();
        let stdin = io::stdin();
        let input_line = stdin.lock().lines().next().unwrap().unwrap();
        if match_pattern(&input_line, &pattern) {
            process::exit(0);
        } else {
            process::exit(1);
        }
    }
}
