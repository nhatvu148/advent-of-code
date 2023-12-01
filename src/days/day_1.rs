use std::fs::File;
use std::io::{self, BufRead};

#[cfg(test)]
#[path = "../unit_tests/day_1.rs"]
mod day_1_tests;

pub fn extract_first_and_last_digits(input: &str) -> Option<u32> {
    let digits: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    if let Some(first) = digits.first().cloned() {
        if let Some(last) = digits.last().cloned() {
            return Some(first * 10 + last);
        }
    }
    None
}

pub fn process_file(file_path: &str) -> io::Result<u32> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut total: u32 = 0;

    for line in reader.lines() {
        if let Ok(line_content) = line {
            if let Some(combined_digits) = extract_first_and_last_digits(&line_content) {
                // println!(
                //     "Input: {:?}, Combined Digits: {}",
                //     line_content, combined_digits
                // );
                total += combined_digits;
            } else {
                println!("Input: {:?}, No digits found", line_content);
            }
        }
    }

    Ok(total)
}
