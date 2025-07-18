use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

fn main() {
    let file = File::open("non_existent_file.txt");
    let file = match file {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                panic!("File not found: {}", e);
            }
            ErrorKind::PermissionDenied => {
                panic!("Permission denied: {}", e);
            }
            _ => {
                panic!("An error occurred: {}", e);
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap())
    }
}
