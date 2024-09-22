use std::env;
use std::io;
use std::process;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Clone, PartialEq)]
enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
    StartOfLine,
    EndOfLine,
    OneOrMore(Box<Pattern>),
}

fn match_literal(chars: &mut Peekable<std::slice::Iter<'_, char>>, literal: char) -> bool {
    chars.next().map_or(false, |&c| c == literal)
}

fn match_digit(chars: &mut Peekable<std::slice::Iter<'_, char>>) -> bool {
    chars.next().map_or(false, |&c| c.is_digit(10))
}

fn match_alphanumeric(chars: &mut Peekable<std::slice::Iter<'_, char>>) -> bool {
    chars.next().map_or(false, |&c| c.is_alphanumeric())
}

fn match_group(chars: &mut Peekable<std::slice::Iter<'_, char>>, group: &str, positive: bool) -> bool {
    chars.next().map_or(false, |&c| group.contains(c) == positive)
}

fn match_start_of_line(start: usize) -> bool {
    start == 0
}

fn match_end_of_line(chars: &mut Peekable<std::slice::Iter<'_, char>>) -> bool {
    chars.peek().is_none()
}

fn match_one_or_more(chars: &mut Peekable<std::slice::Iter<'_, char>>, pattern: &Pattern) -> bool {
    let mut matched = false;
    while match_pattern_helper(chars, &[pattern.clone()]) {
        matched = true;
    }
    matched
}

// function to match a pattern with the input line
fn match_pattern_helper(chars: &mut Peekable<std::slice::Iter<'_, char>>, pattern: &[Pattern]) -> bool {
    let mut char_iter = chars.clone();
    for pat in pattern {
        let matched = match pat {
            Pattern::Literal(l) => match_literal(&mut char_iter, *l),
            Pattern::Digit => match_digit(&mut char_iter),
            Pattern::Alphanumeric => match_alphanumeric(&mut char_iter),
            Pattern::Group(positive, group) => match_group(&mut char_iter, group, *positive),
            Pattern::StartOfLine => true, 
            Pattern::EndOfLine => char_iter.peek().is_none(),
            Pattern::OneOrMore(p) => match_one_or_more(&mut char_iter, p),
        };
        if !matched {
            return false;
        }
    }
    *chars = char_iter;
    true
}

fn match_pattern(input_line: &str, pattern: &[Pattern]) -> bool {
    let input_chars: Vec<char> = input_line.chars().collect();
    let input_len = input_chars.len();
    let starts_with_anchor = pattern.first() == Some(&Pattern::StartOfLine);
    let ends_with_anchor = pattern.last() == Some(&Pattern::EndOfLine);

    let start_range = if starts_with_anchor { 0..1 } else { 0..input_len + 1 };

    for start in start_range {
        let mut iter = input_chars[start..].iter().peekable();
        let effective_pattern = if starts_with_anchor { &pattern[1..] } else { pattern };
        if match_pattern_helper(&mut iter, effective_pattern) {
            if !ends_with_anchor || iter.peek().is_none() {
                return true;
            }
        }
    }

    false
}

// helper to build character group patterns like [a-z]
fn build_char_group_patterns(iter: &mut Peekable<Chars>) -> Result<(bool, String), String> {
    let mut group = String::new();
    let mut positive = true;

    if iter.peek() == Some(&'^') {
        positive = false;
        iter.next();
    }

    for member in iter.by_ref() {
        if member == ']' {
            return Ok((positive, group));
        }
        group.push(member);
    }

    Err("incomplete character group".to_string())
}

fn build_patterns(pattern: &str) -> Result<Vec<Pattern>, String> {
    let mut iter = pattern.chars().peekable();
    let mut patterns = Vec::new();

    while let Some(current) = iter.next() {
        let mut pat = match current {
            '^' => Pattern::StartOfLine,
            '$' => Pattern::EndOfLine,
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

        if iter.peek() == Some(&'+') {
            iter.next();
            pat = Pattern::OneOrMore(Box::new(pat));
        }

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

    if match_pattern(input_line.trim(), &patterns) {
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
