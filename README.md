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

```
