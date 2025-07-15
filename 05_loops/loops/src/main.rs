fn main() {
    let mut x = 1;

    loop {
        x += 1; // Increment x
        if x > 5 {
            break; // Exit the loop when x is greater than 5
        }
        println!("Current value of x: {}", x);
    }

    let x = Some(42);
    if let Some(value) = x {
        println!("Value is: {}", value); // This will print 42
    } else {
        println!("No value found");
    }
}
