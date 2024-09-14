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
                // handle escaped sequences 
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
                        _ => return false, // unsupported escape sequence
                    }
                } else {
                    return false; // dangling escape character
                }
            }
            '[' => {
                // handle character classes
                let mut char_class = Vec::new();
                let mut negate = false;

                if let Some(next_char) = pattern_chars.peek() {
                    if *next_char == '^' {
                        negate = true;
                        pattern_chars.next(); // Skip the '^'
                    }
                }

                while let Some(class_char) = pattern_chars.next() {
                    if class_char == ']' {
                        break;
                    }
                    char_class.push(class_char);
                }

                if let Some(input_char) = input_chars.next() {
                    let mut matched = false;
                    let mut i = 0;

                    while i < char_class.len() {
                        if i + 2 < char_class.len() && char_class[i + 1] == '-' {
                            // handle ranges like a-z
                            if input_char >= char_class[i] && input_char <= char_class[i + 2] {
                                matched = true;
                                break;
                            }
                            i += 3;
                        } else {
                            // handle single characters
                            if input_char == char_class[i] {
                                matched = true;
                                break;
                            }
                            i += 1;
                        }
                    }

                    if negate {
                        if matched {
                            return false;
                        }
                    } else {
                        if !matched {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
            _ => {
                // match literal characters
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

    input_chars.next().is_none()
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
