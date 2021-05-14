// See https://stackoverflow.com/questions/37531903/how-do-i-print-output-without-a-trailing-newline-in-rust
// to see why it is necessary to use std::io::Write
use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;

pub const ERR_PREFIX: &str = " *** Error:";

/// Prints a string to standard output without printing a new line, and
/// flushes the output. This makes it suitable for input prompts.
pub fn print_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().expect("Failed to flush output.");
}

pub fn print_err(message: &str) {
    println!("{} {}.", ERR_PREFIX, message);
}

/// Reads a line from standard input, and removes leading/trailing
/// whitespace.
pub fn input_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line.");

    s.trim().to_string()
}

pub fn halt() {
    print_flush("Press return to continue");
    input_line();
}

pub fn input<T: FromStr>() -> Result<T, ()> {
    let s = input_line();
    match s.parse() {
        Ok(data) => Ok(data),

        Err(_) => Err(()),
    }
}

pub fn input_within_range<T>(min_valid: T, max_valid: T) -> Result<T, ()>
where
    T: FromStr + PartialOrd + Copy,
{
    let data = input()?;
    if data < min_valid || data > max_valid {
        Err(())
    } else {
        Ok(data)
    }
}

pub fn input_loop<T: FromStr>(prompt: &str) -> T {
    loop {
        print_flush(prompt);
        match input() {
            Ok(data) => {
                return data;
            }

            Err(_) => {
                print_err("data entered is invalid/outside of range");
            }
        }
    }
}

pub fn input_within_range_loop<T>(prompt: &str, min_valid: T, max_valid: T) -> T
where
    T: fmt::Display + FromStr + PartialOrd + Copy,
{
    loop {
        print_flush(prompt);
        match input_within_range(min_valid, max_valid) {
            Ok(data) => {
                return data;
            }

            Err(_) => {
                println!(
                    "{} number must be between {} and {}.",
                    ERR_PREFIX, min_valid, max_valid
                );
            }
        }
    }
}
