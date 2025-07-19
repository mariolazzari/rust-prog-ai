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

### Function

```rust
fn main() {
    process_numbers(&[1, 2, 3, 4, 5]);

    let chunk = split_string("Hello,World,This,Is,Rust".to_string(), ',', 2);
    println!("The selected chunk is: {}", chunk);
}

fn process_numbers(nums: &[i32]) {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }

    println!("The sum of the numbers is: {}", sum);

    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}

fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let res = parts.get(field);

    return res.expect("Something went wrong...").to_string();
}
```

### Borrowig

```rust
fn own_vector(vector: &mut Vec<i32>) {
    vector.push(4);
    println!("Vector inside function: {:?}", vector);
}

fn own_integer(mut value: i32) -> i32 {
    value += 1;
    println!("Integer inside function: {}", value);
    value
}

fn own_string(s: &String) {
    println!("String inside function: {}", s);
}

fn main() {
    let mut my_vector = vec![1, 2, 3];
    let my_int = 10;
    let my_string = String::from("Hello, Rust!");

    own_integer(my_int);
    println!("Integer after function call: {}", my_int);

    // own_string(my_string);
    // println!("String after function call: {}", my_string);
    // This line would cause a
    // compile-time error because `my_string` is moved into `own_string`.

    // borrow string
    own_string(&my_string);

    own_vector(&mut my_vector);
    println!("Vector after function call: {:?}", my_vector);
}
```

### Panic

```rust
fn main() {
    loop_and_panic(vec![1, 2, -3, 4, 5]);
}

fn loop_and_panic(nums: Vec<i32>) {
    loop {
        for n in &nums {
            if *n < 0 {
                panic!("Negative number encountered: {}", n);
            }
        }
    }
}
```

### Error handling with match

```rust
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
```

## Structured data

### Structs

```rust
#[derive(Debug)]

struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

fn main() {
    let person = Person {
        first_name: "Mario".to_string(),
        last_name: "Lazzari".to_string(),
        age: Some(50),
    };

    println!("{:?}", person);
    println!(
        "My name is {} {} and I'm {:?}.",
        person.first_name,
        person.last_name,
        person.age.unwrap_or(0)
    );

    let person = Person {
        first_name: "Maria".to_string(),
        last_name: "Lazzari".to_string(),
        age: None,
    };

    println!("{:?}", person);
    println!("My name is {} {}", person.first_name, person.last_name);
}
```

### Associated functions

```rust
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

struct Point(i32, i32, i32);

fn main() {
    let mut user = User::new(
        String::from("mariolazzari"),
        String::from("mario.lazzari@gmail.com"),
        String::from("https://mariolazzari.it"),
    );
    println!("User {} is active? {}", user.username, user.active);

    user.deactivate();
    println!("User {} is active? {}", user.username, user.active);

    let point = Point(1, 2, 3);
    println!("My point: ({},{},{})", point.0, point.1, point.2);
}
```

## Strings and vectors

### Strings and string slices

```rust
fn print_str(s: &str) {
    println!("Print {}", s)
}

fn print_string(s: String) {
    println!("Print {}", s)
}

fn main() {
    // string size known at compilation time
    let s = "String slice";
    print_str(s);

    // dynamic string size
    let s = String::from("String");
    print_string(s);

    // string manipulation
    let sentence = "Ciao io sono Mario Lazzari.";
    println!("{}", sentence);

    // string iteration
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("vowel {}", c),
            _ => continue,
        }
    }

    // get first 3 chars of a slice
    println!("{}", &sentence[0..=3]);

    // split string into a vector
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);

    // reverse string
    let reversed = sentence.chars().rev().collect::<String>();
    println!("{:?}", reversed);
}
```

### Vectors

```rust
fn main() {
    ownership();
    modifiable();
}

fn ownership() {
    let numbers = vec![1, 2, 3];
    let slice = &numbers[..];
    println!("slice {:?}", slice);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3];
    let slice = &mut numbers[..];
    slice[0] = 10;
    println!("slice {:?}", slice);
}
```
