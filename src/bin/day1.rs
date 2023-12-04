use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

static NUMBER_MAP: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
    HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
});

fn parse_line(input: &str) -> u32 {
    let mut digits: Vec<u32> = Vec::new();
    let input = input.as_bytes();

    let len = input.len();
    let mut i = 0;
    while i < input.len() {
        match (input[i] as char).to_digit(10) {
            Some(d) => {
                digits.push(d);
            }
            None => {
                for (word, value) in &(*NUMBER_MAP) {
                    if i + word.len() <= len && &input[i..i + word.len()] == word.as_bytes() {
                        digits.push(*value);
                        break;
                    }
                }
            }
        }
        i += 1;
    }

    digits[0] * 10 + digits[digits.len() - 1]
}

fn calibrate() -> u32 {
    let file = File::open("input/day1.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    let sum: u32 = reader
        .lines()
        .filter_map(|p| p.ok())
        .map(|line| {
            let number = parse_line(&line);
            println!("line: {} digits: {}", line, number);
            number
        })
        .sum();
    println!("Sum: {}", sum);
    sum
}

fn main() {
    calibrate();
}
