///! This module handles CSV conversion of a list of passwords

use serde::Serialize;
use std::{error::Error, fs::File, path::PathBuf};

/// CSV record structure, holding password-related data
#[derive(Debug, Serialize)]
struct Record {
    password: String,
    length: String,
    num_chars: String,
    num_digits: String,
    num_upper: String,
    num_lower: String,
    num_special: String,
    num_vowels: String,
    class: String,
}

/// Implementation of the Default trait for Record.
/// Returns an empty row.
impl Default for Record {
    fn default() -> Record {
        Record {
            password: String::from(""),
            length: String::from(""),
            num_chars: String::from(""),
            num_digits: String::from(""),
            num_upper: String::from(""),
            num_lower: String::from(""),
            num_special: String::from(""),
            num_vowels: String::from(""),
            class: String::from(""),
        }
    }
}

/// Write a password vector to a CSV-formtatted file
pub fn write_csv(passwords: &Vec<String>, filename: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut wtr = csv::Writer::from_writer(file);
    passwords.iter().for_each(|password| {
        wtr.serialize(Record {
            password: String::from(password),
            ..Default::default()
        })
        .unwrap();
    });
    wtr.flush()?;
    Ok(())
}
