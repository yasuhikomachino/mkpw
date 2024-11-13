use clap::Parser;

mod generator;

const LENGTH_MIN: usize = 3;
const NUMBER_MIN: usize = 1;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(short = 'l', long, default_value_t = 12, value_parser = validate_length, help = "Length of password (minimum 3 characters)")]
    length: usize,

    #[clap(short = 'n', long, default_value_t = 1, value_parser = validate_number, help = "Number of passwords to generate")]
    number: usize,

    #[clap(short = 'u', long, help = "Include uppercase letters")]
    uppercase: bool,

    #[clap(short = 'd', long, help = "Include numbers")]
    digits: bool,

    #[clap(short = 's', long, help = "Include symbols")]
    symbols: bool,
}

fn validate_length(s: &str) -> Result<usize, String> {
    let value: usize = s.parse().map_err(|_| format!("Invalid number: {}", s))?;
    if value < LENGTH_MIN {
        Err(format!(
            "Password length must be at least {} characters",
            LENGTH_MIN
        ))
    } else {
        Ok(value)
    }
}

fn validate_number(s: &str) -> Result<usize, String> {
    let value: usize = s.parse().map_err(|_| format!("Invalid number: {}", s))?;
    if value < NUMBER_MIN {
        Err(format!("Password number must be at least {}", NUMBER_MIN))
    } else {
        Ok(value)
    }
}

fn main() {
    let args = Args::parse();
    let g = generator::Password::new(args.uppercase, args.symbols, args.digits);

    for _ in 0..args.number {
        println!("{}", g.execute(args.length));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_length_valid() {
        assert!(validate_length("3").is_ok());
        assert!(validate_length("12").is_ok());
    }

    #[test]
    fn validate_length_invalid() {
        assert!(validate_length("0").is_err());
        assert!(validate_length("2").is_err());
        assert!(validate_length("-1").is_err());
        assert!(validate_length("abc").is_err());
    }
}
