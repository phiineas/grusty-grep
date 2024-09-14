use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else if pattern == "\\d" {
        return input_line.contains(|c: char| c.is_digit(10));
    } else if pattern == "\\w" {
        return input_line.contains(|c: char| c.is_alphanumeric());
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        let mut new_pattern = pattern.trim_matches('[').trim_matches(']').bytes();
        input_line.bytes().any(|val| new_pattern.any(|p| val == p))
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        let pos_pattern: Vec<char> = pattern[1..pattern.len()-1].chars().collect();
        input_line.chars().any(|c| pos_pattern.contains(&c))
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// usage- echo <input_text> | your_program.sh -E <pattern>

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "-E" {
        println!("Usage: your_program -E <pattern>");
        process::exit(1);
    }

    let pattern = &args[2];
    let stdin = io::stdin();
    let input_line = stdin.lock().lines().next().unwrap().unwrap();

    if match_pattern(&input_line, pattern) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
