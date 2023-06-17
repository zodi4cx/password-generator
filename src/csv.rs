use passwords::{analyzer, scorer};
///! This module handles CSV conversion of a list of passwords
use serde::Serialize;
use std::{error::Error, fs::File, path::PathBuf};

/// CSV record structure, holding password-related data
#[derive(Debug, Serialize)]
struct Record {
    password: String,
    length: usize,
    num_chars: usize,
    num_digits: usize,
    num_upper: usize,
    num_lower: usize,
    num_special: usize,
    num_vowels: usize,
    score: usize,
    class: String,
}

impl Record {
    fn new(password: &str) -> Record {
        let analyzed = analyzer::analyze(password);
        let vowels_count = password
            .chars()
            .filter(|c| "aeiouAEIOU".contains(*c))
            .count();
        Record {
            password: String::from(password),
            length: analyzed.length(),
            num_chars: analyzed.lowercase_letters_count() + analyzed.uppercase_letters_count(),
            num_digits: analyzed.numbers_count(),
            num_upper: analyzed.uppercase_letters_count(),
            num_lower: analyzed.lowercase_letters_count(),
            num_special: analyzed.symbols_count(),
            num_vowels: vowels_count,
            score: scorer::score(&analyzed) as usize,
            class: String::new(),
        }
    }
}

/// Write a password vector to a CSV-formtatted file
pub fn write_csv(passwords: &Vec<String>, filename: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut wtr = csv::Writer::from_writer(file);
    passwords.iter().for_each(|password| {
        wtr.serialize(Record::new(&password)).unwrap();
    });
    wtr.flush()?;
    Ok(())
}
