use clap::Parser;
use itertools::Itertools;
use passwords::PasswordGenerator;
use std::{error::Error, path::PathBuf};

mod csv;

#[derive(Parser)]
pub struct Args {
    /// Minimum password length
    #[arg(default_value_t = 5)]
    min_length: usize,

    /// Maximum password length
    #[arg(default_value_t = 20)]
    max_length: usize,

    /// Number of each password type to generate
    #[arg(default_value_t = 50)]
    repetition: usize,

    /// Output CSV file
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

/// PasswordGenerator with custom default values
const DEFAULT_GENERATOR: PasswordGenerator = PasswordGenerator {
    length: 8,
    numbers: true,
    lowercase_letters: true,
    uppercase_letters: true,
    symbols: true,
    spaces: false,
    exclude_similar_characters: false,
    strict: true,
};

/// Runtime to be executed by the crate binary
pub fn run(config: Args) -> Result<(), Box<dyn Error>> {
    let passwords = generate_passwords(config.min_length, config.max_length, config.repetition);
    eprintln!("[*] Passwords generated: {}", passwords.len());
    if let Some(output_filename) = config.output {
        csv::write_csv(&passwords, &output_filename)?;
        println!(
            "[+] Output written to {}",
            output_filename.to_string_lossy()
        );
    } else {
        passwords.iter().for_each(|password| println!("{password}"));
    }
    Ok(())
}

/// Generate passwords of varying length and specifications.
pub fn generate_passwords(min_length: usize, max_length: usize, n_passwords: usize) -> Vec<String> {
    let mut passwords = Vec::new();
    let mut options = (0..4)
        .map(|_| [false, true].iter())
        .multi_cartesian_product();
    // At least one kind of characters must be enabled
    options.next();

    for length in min_length..=max_length {
        let options = options.clone();
        for option in options {
            let generator = PasswordGenerator {
                length,
                symbols: *option[0],
                numbers: *option[1],
                uppercase_letters: *option[2],
                lowercase_letters: *option[3],
                ..DEFAULT_GENERATOR
            };
            generator
                .generate(n_passwords)
                .unwrap()
                .iter()
                .for_each(|password| {
                    passwords.push(password.clone());
                });
        }
    }
    passwords
}
