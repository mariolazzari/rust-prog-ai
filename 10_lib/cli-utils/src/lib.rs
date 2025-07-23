//! This is a library for reading from stdin
//! # Examples:
//! ```
//! use std::io::{BufRead, BufReader};
//! ```
//! # Panics:
//! 'read_line' will panic if it fails to read from stdin.
use std::io::{BufRead, BufReader};

/// This function reads a line from stdin and returns a String
/// It will panic if it fails to read a line
/// # Examples:
/// ```
/// let input = read_stdin()
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();

    reader
        .read_line(&mut line)
        .expect("Falied to read input line");

    line.trim().to_string()
}
