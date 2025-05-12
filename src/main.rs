//! # NHS Number command line interface (cli)
//! 
//! A National Health Service (NHS) Number is a unique number allocated in a shared
//! numbering scheme to registered users of the three public health services in
//! England, Wales, and the Isle of Man.
//! 
//! This tool is a command line interface that parses each standard input line into
//! an NHS number, then validates the check digit is correct.
//! 
//! * If the line is a valid NHS number, then print it.
//! 
//! * If the line is an invalid NHS Number, or is unparseable, then print an error message.
//! 
//! * If the line is blank, then skip it.
//! 
//! References:
//! 
//! * [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)
//! 
//! * [NHS Number](https://en.wikipedia.org/wiki/NHS_number)
//! 
//! ## Examples
//! 
//! Suppose you have a text file `input.txt` that contains one NNS Number per line,
//! and some may be valid or invalid:
//!
//! 
//! ```txt
//! 999 123 4560
//! 999 123 4561
//! ```
//! 
//! You can parse each line and validate it:
//! 
//! ```sh
//! cat input.txt | nhs-number-cli
//! ```
//! 
//! The output is one stdout line and one stderr line: 
//! 
//! ```stdout
//! 999 123 4560
//! ```
//! 
//! ```stderr
//! Error invalid line 1. Error: validate check digit failed. NHS Number: 999 123 4561
//! ```
 
use std::io::{self, BufRead};
use nhs_number::NHSNumber;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        match line {
            Ok(line) => {
                if line == "" {
                    continue;
                }
                match NHSNumber::from_str(&line) {
                    Ok(nhs_number) => {
                        match nhs_number.validate_check_digit() {
                            true => {
                                println!("{}", nhs_number)
                            },
                            false => {
                                eprintln!("Error invalid line {}. Error: validate check digit failed. NHS Number: {}", i, &nhs_number)
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error parsing line {}. Error: {:?}. Line: {}", i, e, &line);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading line {}. Error: {}", i, e);
            }
        }
    }
}
