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

```
