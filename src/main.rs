use std::env;
use std::io;
use std::io::Read;
use std::process;
use std::str::Chars;

enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
    StartOfString,
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

fn match_start_of_string(chars: &mut Chars) -> bool {
    chars.as_str().is_empty()
}

fn parse_pattern(pattern: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut chars = pattern.chars();
    while let Some(c) = chars.next() {
        match c {
            '^' => patterns.push(Pattern::StartOfString),
            '0'..='9' => patterns.push(Pattern::Digit),
            'a'..='z' | 'A'..='Z' => patterns.push(Pattern::Alphanumeric),
            _ => patterns.push(Pattern::Literal(c)),
        }
    }
    patterns
}

fn match_pattern(patterns: &[Pattern], input: &str) -> bool {
    let mut chars = input.chars();
    for pattern in patterns {
        let matched = match pattern {
            Pattern::Literal(c) => match_literal(&mut chars, *c),
            Pattern::Digit => match_digit(&mut chars),
            Pattern::Alphanumeric => match_alphanumeric(&mut chars),
            Pattern::Group(positive, group) => match_group(&mut chars, group, *positive),
            Pattern::StartOfString => match_start_of_string(&mut chars),
        };
        if !matched {
            return false;
        }
    }
    true
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
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "-E" {
        eprintln!("Usage: {} -E <pattern>", args[0]);
        process::exit(1);
    }

    let pattern = &args[2];
    let patterns = parse_pattern(pattern);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if match_pattern(&patterns, &input) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
