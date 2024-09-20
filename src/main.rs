use std::env;
use std::io;
use std::process;
use std::str::Chars;

// Enum representing different pattern types
enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
    StartOfLine,
}

// Functions for matching different patterns
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

fn match_start_of_line(chars: &Chars, input_line: &str) -> bool {
    chars.as_str() == input_line
}

// function to match a pattern with the input line
fn match_pattern(input_line: &str, pattern: &[Pattern]) -> bool {
    let mut iter = input_line.chars();

    for pat in pattern {
        let matched = match pat {
            Pattern::Literal(l) => match_literal(&mut iter, *l),
            Pattern::Digit => match_digit(&mut iter),
            Pattern::Alphanumeric => match_alphanumeric(&mut iter),
            Pattern::Group(positive, group) => match_group(&mut iter, group, *positive),
            Pattern::StartOfLine => match_start_of_line(&iter, input_line),
        };

        if !matched {
            return false;
        }
    }

    true
}

// helper to parse character group patterns like [a-z]
fn build_char_group_patterns(iter: &mut Chars) -> Result<(bool, String), String> {
    let mut group = String::new();
    let mut positive = true;

    if iter.clone().next() == Some('^') {
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

// builds pattern sequence from string input (e.g., "^log", "\d", "[a-z]")
fn build_patterns(pattern: &str) -> Result<Vec<Pattern>, String> {
    let mut iter = pattern.chars();
    let mut patterns = Vec::new();

    while let Some(current) = iter.next() {
        let pat = match current {
            '^' => Pattern::StartOfLine,
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
        };
        patterns.push(pat);
    }

    Ok(patterns)
}

// function to handle input processing, pattern building, and matching
fn process_input_and_match() -> Result<(), String> {
    if env::args().nth(1).unwrap_or_default() != "-E" {
        return Err("Expected first argument to be '-E'".to_string());
    }

    let pattern = env::args().nth(2).ok_or_else(|| "Pattern is required".to_string())?;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).map_err(|_| "Failed to read input".to_string())?;

    let patterns = build_patterns(&pattern)?;

    if match_pattern(&input_line.trim(), &patterns) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn main() {
    if let Err(err) = process_input_and_match() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
