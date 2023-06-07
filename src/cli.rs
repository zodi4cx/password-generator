use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use super::csv;

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

/// Runtime to be executed by the crate binary
pub fn run(config: Args) -> Result<(), Box<dyn Error>> {
    let passwords =
        crate::generate_passwords(config.min_length, config.max_length, config.repetition);
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
