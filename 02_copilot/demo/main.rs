// function that returns average of few numbers
fn average(numbers: &[f64]) -> f64 {
    // loop that prints out all numbers
    for &number in numbers.iter() {
        println!("Number: {}", number);
    }

    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

fn main() {
    println!("Hello, world!");
    // This is a simple Rust program that prints "Hello, world!" to the console.
}
