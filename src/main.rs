use std::env;
use std::io;
use std::process;

enum Pattern {
    Digit,
    Alphanumeric,
    Literal(char),
    Group(bool, Vec<char>),
}

fn build_patterns(pattern: &str) -> Vec<Pattern> {
    let mut iter = pattern.chars();
    let mut patterns = Vec::new();

    loop {
        let current = iter.next();
        if current.is_none() {
            break;
        }

        patterns.push(match current.unwrap() {
            '\\' => {
                let special = iter.next();
                if special.is_none() {
                    panic!("Incomplete special character");
                }
                match special.unwrap() {
                    'd' => Pattern::Digit,
                    'w' => Pattern::Alphanumeric,
                    '\\' => Pattern::Literal('\\'),
                    _ => panic!("Invalid special character"),
                }
            }
            '[' => {
                let (positive, group) = build_group_pattern(&mut iter);
                Pattern::Group(positive, group)
            }
            l => Pattern::Literal(l),
        });
    }

    patterns
}

fn build_group_pattern(iter: &mut std::str::Chars) -> (bool, Vec<char>) {
    let mut group = Vec::new();
    let mut positive = true;

    if let Some(&'^') = iter.clone().next() {
        iter.next();
        positive = false;
    }

    while let Some(c) = iter.next() {
        if c == ']' {
            break;
        }
        group.push(c);
    }

    (positive, group)
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let patterns = build_patterns(pattern);

    for pat in patterns {
        match pat {
            Pattern::Digit => {
                if !input_line.chars().any(|c| c.is_digit(10)) {
                    return false;
                }
            }
            Pattern::Alphanumeric => {
                if !input_line.chars().any(|c| c.is_alphanumeric()) {
                    return false;
                }
            }
            Pattern::Literal(l) => {
                if !input_line.contains(l) {
                    return false;
                }
            }
            Pattern::Group(positive, group) => {
                if positive {
                    if !input_line.chars().any(|c| group.contains(&c)) {
                        return false;
                    }
                } else {
                    if input_line.chars().any(|c| group.contains(&c)) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        eprintln!("Usage: echo <input_text> | your_program.sh -E <pattern>");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    if match_pattern(&input, &pattern) {
        println!("Match found");
    } else {
        println!("No match");
    }
}
