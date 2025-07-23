use std::io;

fn main() {
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a work (type 'stop' to exit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        println!("You entered: {}", input);
        // input.clear();
    }
    println!("Bye!")
}
