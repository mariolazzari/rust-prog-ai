use std::io;

fn main() {
    let name = "Mario";

    match name {
        "Mario" => println!("It's me, Mario!"),
        "Luigi" => println!("It's me, Luigi!"),
        _ => println!("Who are you?"),
    }

    println!("Please enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    match name.trim() {
        "mario" => println!("Hi Mario!"),
        "luigi" => println!("Hi Luigi!"),
        _ => println!("Who are you?"),
    }
}
