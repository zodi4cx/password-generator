use serde::Serialize;
use std::{error::Error, fs::File, path::PathBuf};

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
