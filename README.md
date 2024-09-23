# GrustyGrep

GrustyGrep is a simple and efficient grep tool written in Rust. It allows you to search for patterns within text files, leveraging Rust's performance and safety features.

## Features

- Match patterns using regular expressions
- Support for various pattern matching options (wildcards, one or more, zero or one)
- Efficient and fast searching

## Installation

To build GrustyGrep, ensure you have Rust installed. Then, clone the repository and run the following commands:

```sh
git clone https://github.com/phiineas/grusty-grep.git
cd grusty-grep
cargo build --release
```

## Usage
To use GrustyGrep, run the following command:

```sh
echo "<input_text>" | ./your_program.sh -E "<pattern>"
```

For example:
```sh
echo "hello world" | ./target/release/rustygrep -E "h.llo"
```

## Development

### Building the Project

```sh
cargo build
```

### Running Tests

```sh
cargo test
```

## Pattern Matching

### Match a Literal Character
handle the simplest regex possible: a single character.

```rust
Pattern::Literal('a')
```

### Match Digits
`\d` matches any digit.

```rust
Pattern::Digit
```

### Match Alphanumeric Characters
`\w` matches any alphanumeric character (a-z, A-Z, 0-9, _).

```rust
Pattern::Alphanumeric
```

### Positive Character Groups
Positive character groups match any character that is present within a pair of square brackets.

```rust
Pattern::Group(true, "abc".to_string())
```

### Negative Character Groups
Negative character groups match any character that is not present within a pair of square brackets.

```rust
Pattern::Group(false, "xyz".to_string())
```

### Combining Character Classes
Use character classes with other patterns for complex matching.

### Start of String Anchor
`^` doesn't match a character, it matches the start of a line.

```rust
Pattern::StartOfLine
```

### End of String Anchor
`$` doesn't match a character, it matches the end of a line.

```rust
Pattern::EndOfLine
```

### Match One or More Times

```rust
Pattern::OneOrMore(Box::new(Pattern::Literal('a')))
```

### Match Zero or One Times

```rust
Pattern::ZeroOrOne(Box::new(Pattern::Literal('a')))
```

### Wildcard

```rust
Pattern::Wildcard
```

### Alternation

```rust
Pattern::Alternative(vec![vec![Pattern::Literal('a')], vec![Pattern::Literal('b')]])
```

## Future plans

- Single Backreference
- Multiple Backreferences
- Nested Backreferences
