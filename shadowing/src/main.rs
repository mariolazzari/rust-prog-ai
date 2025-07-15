fn main() {
    let x = 5;
    {
        let x = x + 1; // This shadows the outer `x`
        println!("Inner x: {}", x);
    }
    println!("Outer x: {}", x);

    let mut height = 150;
    height += 10; // This modifies the mutable variable

    let result = if height > 160 {
        "Tall" // This is a new variable, not shadowing
    } else {
        "Short"
    };
    println!("Height: {}, Result: {}", height, result);
}
