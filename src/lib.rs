use itertools::Itertools;
use passwords::PasswordGenerator;

pub mod cli;
mod csv;

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
