use clap::Parser;

mod cli;

fn main() {
    let _ = cli::Cli::parse();
}
