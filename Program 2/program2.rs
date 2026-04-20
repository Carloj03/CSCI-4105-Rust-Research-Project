/*
Program Description:

This program reads an integer n from a file provided as a command-line argument,
then reads n integer values from the same file.

It stores the values in a dynamically allocated vector (Vec), computes their sum,
and calculates their average.

The program performs basic validation on command-line arguments, file access,
input parsing, and the value of n. Errors are reported and the program exits
early when invalid input or system failures are encountered.

Memory is managed automatically by Rust’s ownership system; no manual freeing
is required.
*/

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_N: usize = 10_000_000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage_text = "./program2 <input_file.txt>";

    // Error 1: Incorrect number of arguments
    if args.len() != 2 {
        eprintln!("Error: Incorrect usage. Correct Usage: {}", usage_text);
        return;
    }

    // Open file
    let file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error: File could not be opened. Correct Usage: {}", usage_text);
            return;
        }
    };

    let mut reader = BufReader::new(file);

    // Read first value (n)
    let mut first_line = String::new();
    if reader.read_line(&mut first_line).is_err() {
        eprintln!("Error: Failed to read input");
        return;
    }

    let n: usize = match first_line.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: First value in file is not valid");
            return;
        }
    };

    // Error 4: range check
    if n < 1 || n > MAX_N {
        eprintln!("Error: n is out of bounds. Must be in [1, 10^7]");
        return;
    }

    let mut numbers: Vec<i32> = Vec::with_capacity(n);
    let mut sum: i64 = 0;

    // Read remaining numbers
    for (idx, line) in reader.lines().enumerate() {
        if idx >= n {
            break;
        }

        let line = match line {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Error: Failed reading input");
                return;
            }
        };

        let value: i32 = match line.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Error: Incorrect value in input file");
                return;
            }
        };

        numbers.push(value);
        sum += value as i64;
    }

    if numbers.len() != n {
        eprintln!("Error: Not enough input values provided");
        return;
    }

    let average = sum as f64 / n as f64;

    // Output
    println!("Sum = {}", sum);
    println!("Average = {:.2}", average);
}