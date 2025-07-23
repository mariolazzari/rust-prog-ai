use clap::Parser;
use resplit::Cli;

fn main() {
    let cli = Cli::parse();
    let buffer = resplit::read_stdin();

    let result = resplit::split(buffer, &cli);
    println!("{}", result);
}
