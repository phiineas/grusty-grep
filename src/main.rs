use std::env;
use std::io;
use std::process;

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_alphanumeric(c: char) -> bool {
    c.is_alphanumeric()
}

fn match_pattern(input: &str, pattern: &str) -> bool {
    let mut input_chars = input.chars().peekable();
    let mut pattern_chars = pattern.chars().peekable();

    while let Some(pat) = pattern_chars.next() {
        match pat {
            '\\' => {
                // handle escaped sequences like \d, \w, etc.
                if let Some(next_pat) = pattern_chars.next() {
                    match next_pat {
                        'd' => {
                            if let Some(input_char) = input_chars.next() {
                                if !is_digit(input_char) {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                        'w' => {
                            if let Some(input_char) = input_chars.next() {
                                if !is_alphanumeric(input_char) {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                        _ => {
                            panic!("Unsupported escape sequence: \\{}", next_pat);
                        }
                    }
                } else {
                    panic!("Incomplete escape sequence");
                }
            }
            '[' => {
                // handle character groups
                let mut char_group = String::new();
                let mut negate = false;

                // Check if it's a negative character group
                if let Some('^') = pattern_chars.peek() {
                    negate = true;
                    pattern_chars.next(); // Skip '^'
                }

                while let Some(c) = pattern_chars.next() {
                    if c == ']' {
                        break;
                    }
                    char_group.push(c);
                }

                if let Some(input_char) = input_chars.next() {
                    let is_match = char_group.contains(input_char);
                    if negate {
                        if is_match {
                            return false;
                        }
                    } else {
                        if !is_match {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
            _ => {
                if let Some(input_char) = input_chars.next() {
                    if input_char != pat {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }

    input_chars.peek().is_none()
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();
    input_line = input_line.trim().to_string(); 

    if match_pattern(&input_line, &pattern) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
