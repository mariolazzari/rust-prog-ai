fn main() {
    // This is a simple for loop that iterates from 1 to 9
    for i in 1..10 {
        println!("The value of i is: {}", i);
    }

    // This is a for loop that iterates from 1 to 10, inclusive
    for i in 1..=10 {
        println!("The value of i is: {:}", i);
    }

    // This is a for loop that iterates over an array
    for i in (1..5).rev() {
        println!("The value of i is: {}", i);
    }

    // This is a for loop that iterates over an array of numbers
    let numbers = vec![10, 20, 30, 40, 50];
    for num in numbers {
        println!("The value of num is: {}", num);
    }

    for i in 1..=10 {
        if i % 2 == 0 {
            // Skip even numbers
            continue;
        }

        if i == 7 {
            // Break the loop when i is 7
            break;
        }

        println!("The value of i is: {}", i);
    }
}
