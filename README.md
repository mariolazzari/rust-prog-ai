# Rust Programming: From Fundamentals to Advanced Concepts with AI-Assisted Development

## Intoduction

### Installing Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### VS Code

### Rust analyzer

### Settings sync

## Github Copilot

### Extension

### Chat

## Codespaces

## Rust introduction

### Create project

```sh 
cargo init .
```

### Create library

```sh
cargo init --lib .
```

### Create named project

```sh
cargo new project
```

### Create named library

```sh
cargo new --lib project
```

### use

```rust
use clap::Parser;
use replit::Cli;

fn main() {
    let cli = Cli::parse();
    let buffer = replit::stdin();

    let res = replit::split(buffer, &cli);
    println!("{}", res);
}
```

### Varaibles

```rust
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
```

### Control flow

```rust
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
```

### Shadowing

```rust
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
```

## Loops

### Loop

```rust
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
```

### While

```rust
use std::io;

fn main() {
    let mut i: i32 = 0;
    while i < 5 {
        println!("i is: {}", i);
        i += 1;
    }

    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word ('stop' to exit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You entered: {}", input.trim());
    }
    println!("Goodbye!");
}
```

### For

```rust
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
```

### Match

```rust
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
```

## Functions

### 