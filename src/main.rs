use clap::Parser;
use password_generator::Args;
use std::process;

fn main() {
    let config = Args::parse();
    password_generator::run(config).unwrap_or_else(|error| {
        println!("[!] An error occured: {error}");
        process::exit(1);
    });
}
