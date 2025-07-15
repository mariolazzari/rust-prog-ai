// use clap::Parser;
use resplit::{read_stdin, split, Cli};

fn main() {
    let cli = Cli::parse();
    let buffer = read_stdin();

    let res = split(buffer, &cli);
    println!("{}", res.to_string());
}
