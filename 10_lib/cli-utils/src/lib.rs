use std::io::{BufRead, BufReader};

pub fn read_stdid() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();

    reader
        .read_line(&mut line)
        .expect("Falied to read input line");

    line.trim().to_string()
}
