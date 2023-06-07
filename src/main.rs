use clap::Parser;
use password_generator::cli;
use password_generator::cli::Args;
use std::process;

fn main() {
    let config = Args::parse();
    cli::run(config).unwrap_or_else(|error| {
        println!("[!] An error occured: {error}");
        process::exit(1);
    });
}
