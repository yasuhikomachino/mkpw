use rand::{seq::SliceRandom, thread_rng, Rng};

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

pub struct Password {
    uppercase: bool,
    digits: bool,
    symbols: bool,
}

impl Password {
    pub fn new(uppercase: bool, symbols: bool, digits: bool) -> Self {
        Password {
            uppercase,
            digits,
            symbols,
        }
    }

    pub fn execute(&self, length: usize) -> String {
        let mut charset = LOWERCASE.to_string();
        if self.uppercase {
            charset.push_str(UPPERCASE);
        }
        if self.digits {
            charset.push_str(DIGITS);
        }
        if self.symbols {
            charset.push_str(SYMBOLS);
        }

        let mut rng = thread_rng();
        let mut password = String::with_capacity(length);
        let mut remaining_length = length;

        // add required characters
        if self.uppercase && remaining_length > 0 {
            password.push(self.get_random_char(UPPERCASE, &mut rng));
            remaining_length -= 1;
        }
        if self.digits && remaining_length > 0 {
            password.push(self.get_random_char(DIGITS, &mut rng));
            remaining_length -= 1;
        }
        if self.symbols && remaining_length > 0 {
            password.push(self.get_random_char(SYMBOLS, &mut rng));
            remaining_length -= 1;
        }

        // add remaining characters at random
        for _ in 0..remaining_length {
            let idx = rng.gen_range(0..charset.len());
            password.push(charset.chars().nth(idx).unwrap());
        }

        // shuffle password characters
        let mut password_chars: Vec<char> = password.chars().collect();
        password_chars.shuffle(&mut rng);
        password_chars.iter().collect()
    }

    fn get_random_char(&self, charset: &str, rng: &mut impl Rng) -> char {
        charset
            .chars()
            .nth(rng.gen_range(0..charset.len()))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_length() {
        let generator = Password::new(false, false, false);
        let password = generator.execute(12);
        assert_eq!(password.len(), 12);
    }

    #[test]
    fn uppercase_inclusion() {
        let generator = Password::new(true, false, false);
        let password = generator.execute(12);
        assert!(password.chars().any(|c| c.is_uppercase()));
    }

    #[test]
    fn digits_inclusion() {
        let generator = Password::new(false, true, false);
        let password = generator.execute(12);
        assert!(password.chars().any(|c| c.is_numeric()));
    }

    #[test]
    fn symbols_inclusion() {
        let generator = Password::new(false, false, true);
        let password = generator.execute(12);
        assert!(password.chars().any(|c| !c.is_alphanumeric()));
    }

    #[test]
    fn all_character_types() {
        let generator = Password::new(true, true, true);
        let password = generator.execute(12);
        assert!(password.chars().any(|c| c.is_uppercase()));
        assert!(password.chars().any(|c| c.is_numeric()));
        assert!(password.chars().any(|c| !c.is_alphanumeric()));
    }

    #[test]
    fn minimum_length_with_all_options() {
        let generator = Password::new(true, true, true);
        let password = generator.execute(3);
        assert_eq!(password.len(), 3);
        assert!(password.chars().any(|c| c.is_uppercase()));
        assert!(password.chars().any(|c| c.is_numeric()));
        assert!(password.chars().any(|c| !c.is_alphanumeric()));
    }
}
