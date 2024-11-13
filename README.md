# mkpw

A simple password generator CLI tool.

## Features

- Generate random passwords with customizable length
- Multiple password generation at once
- Configurable character sets:
  - Lowercase letters (always included)
  - Uppercase letters (optional)
  - Numbers (optional)
  - Special symbols (optional)

## Requirements

- Rust 1.82.0 or higher
- Cargo

## Installation

```
git clone https://github.com/yasuhikomachino/mkpw
cd mkpw
cargo install --path .
```

## Usage

```
Usage: mkpw [OPTIONS]

Options:
  -l, --length <LENGTH>  Length of password (minimum 3 characters) [default: 12]
  -n, --number <NUMBER>  Number of passwords to generate [default: 1]
  -u, --uppercase        Include uppercase letters
  -d, --digits           Include numbers
  -s, --symbols          Include symbols
  -h, --help             Print help
  -V, --version          Print version
```

## Examples

```
# Generate a basic 12-character password
$ mkpw
rtvaahqhqtzm

# Generate a 16-character password with all character types
$ mkpw -l 16 -u -d -s
sw?6:H!X7YDvrn7Q

# Generate 3 different passwords
$ mkpw -n 3
tebyqmetvbcj
xhtueiweqfnl
ueioamtpuxsg
```
