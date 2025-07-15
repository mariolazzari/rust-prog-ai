fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding with the operation.");
    } else {
        println!("Operation cancelled.");
    }

    let number = 5;
    if number < 10 {
        println!("The number is less than 10.");
    } else if number == 10 {
        println!("The number is exactly 10.");
    } else {
        println!("The number is greater than 10.");
    }
}
