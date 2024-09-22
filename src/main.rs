use std::env;
use std::io;
use std::process;
use std::str::Chars;

enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
    StartOfLine,
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

fn match_start_of_line(chars: &Chars, input_line: &str) -> bool {
    chars.as_str() == input_line
}

// function to match a pattern with the input line
fn match_pattern(input_line: &str, pattern: &[Pattern]) -> bool {
    let input_chars: Vec<char> = input_line.chars().collect();
    let input_len = input_chars.len();

    for start in 0..input_len {
        let mut iter = input_chars[start..].iter().peekable();
        let mut all_matched = true;

        for pat in pattern {
            let matched = match pat {
                Pattern::Literal(l) => iter.next().map_or(false, |&c| c == *l),
                Pattern::Digit => iter.next().map_or(false, |&c| c.is_digit(10)),
                Pattern::Alphanumeric => iter.next().map_or(false, |&c| c.is_alphanumeric()),
                Pattern::Group(positive, group) => iter.next().map_or(false, |&c| group.contains(c) == *positive),
                Pattern::StartOfLine => start == 0,
            };

            if !matched {
                all_matched = false;
                break;
            }
        }

        if all_matched {
            return true;
        }
    }

    false
}

// helper to build character group patterns like [a-z]
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

// function to build patterns from the input string
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
                _ => return Err("invalid special character".to_string()),
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

fn process_input_and_match() -> Result<(), String> {
    if env::args().nth(1).unwrap_or_default() != "-E" {
        return Err("expected first argument to be '-E'".to_string());
    }

    let pattern = env::args().nth(2).ok_or_else(|| "pattern is required".to_string())?;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).map_err(|_| "failed to read input".to_string())?;

    let patterns = build_patterns(&pattern)?;

    if match_pattern(&input_line.trim(), &patterns) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

// usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if let Err(err) = process_input_and_match() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
