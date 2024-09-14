use std::env;
use std::io;
use std::process;
use std::str::Chars;

enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
}

fn match_literal(chars: &mut Chars, literal: char) -> bool {
    chars.next().map_or(false, |c| c == literal)
}

fn match_digit(chars: &mut Chars) -> bool {
    chars.next().map_or(false, |c| c.is_digit(10))
}

fn match_alphanumeric(chars: &mut Chars) -> bool {
    chars.next().map_or(false, |c| c.is_alphanumeric())
}

fn match_group(chars: &mut Chars, group: &str, positive: bool) -> bool {
    chars.next().map_or(false, |c| group.contains(c) == positive)
}

fn match_pattern(input_line: &str, pattern: &[Pattern]) -> bool {
    for i in 0..input_line.len() {
        let mut iter = input_line[i..].chars();
        let mut matched = true;

        for pat in pattern {
            matched = match pat {
                Pattern::Literal(l) => match_literal(&mut iter, *l),
                Pattern::Digit => match_digit(&mut iter),
                Pattern::Alphanumeric => match_alphanumeric(&mut iter),
                Pattern::Group(positive, group) => match_group(&mut iter, group, *positive),
            };

            if !matched {
                break;
            }
        }

        if matched {
            return true;
        }
    }
    false
}

fn build_char_group_patterns(iter: &mut Chars) -> Result<(bool, String), String> {
    let mut group = String::new();
    let mut positive = true;

    if let Some('^') = iter.clone().next() {
        positive = false;
        iter.next();
    }

    for member in iter {
        if member == ']' {
            return Ok((positive, group));
        }
        group.push(member);
    }

    Err("Incomplete character group".to_string())
}

fn build_patterns(pattern: &str) -> Result<Vec<Pattern>, String> {
    let mut iter = pattern.chars();
    let mut patterns = Vec::new();

    while let Some(current) = iter.next() {
        patterns.push(match current {
            '\\' => match iter.next() {
                Some('d') => Pattern::Digit,
                Some('w') => Pattern::Alphanumeric,
                Some('\\') => Pattern::Literal('\\'),
                _ => return Err("Invalid special character".to_string()),
            },
            '[' => {
                let (positive, group) = build_char_group_patterns(&mut iter)?;
                Pattern::Group(positive, group)
            }
            l => Pattern::Literal(l),
        });
    }

    Ok(patterns)
}

// usage: echo <input_text> | your_program.sh -E <pattern>

fn main() {
    if env::args().nth(1).unwrap_or_default() != "-E" {
        eprintln!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap_or_else(|| {
        eprintln!("Pattern is required");
        process::exit(1);
    });

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read input");

    let patterns = build_patterns(&pattern).unwrap_or_else(|err| {
        eprintln!("Pattern error: {}", err);
        process::exit(1);
    });

    if match_pattern(&input_line.trim(), &patterns) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
