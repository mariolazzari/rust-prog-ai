fn main() {
    // unmutable variable
    let message = String::from("Hello, world!");
    let weight: f64 = 75.5; // f64 type
    println!("{} {}", message, weight);

    // mutable variable
    let mut mutable_message = String::from("Hello, mutable world!");
    mutable_message.push_str(" Let's change it.");
    println!("{}", mutable_message);
    mutable_message.clear(); // clear the mutable variable
    println!("After clearing: {}", mutable_message);
}
