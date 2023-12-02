use std::collections::{BTreeMap, HashMap};
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
            let modified_input = find_and_replace_substrings(line_content.as_str());

            if let Some(combined_digits) = extract_first_and_last_digits(&modified_input) {
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

pub fn create_word_to_number_map() -> HashMap<String, u32> {
    let mut map = HashMap::new();

    // Add mappings for individual numbers
    let singles = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    for (index, &word) in singles.iter().enumerate() {
        map.insert(word.to_string(), (index + 1) as u32);
    }

    // Add mappings for tens
    let tens = vec![
        "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    for (index, &word) in tens.iter().enumerate() {
        map.insert(word.to_string(), ((index + 1) * 10).try_into().unwrap());
    }

    // Add mappings for special numbers (eleven to nineteen)
    let specials = vec![
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    for (index, &word) in specials.iter().enumerate() {
        map.insert(word.to_string(), index as u32 + 11);
    }

    map
}

pub fn find_all_substrings_in_array(input: &str, substrings: &[&str]) -> BTreeMap<usize, String> {
    let mut map = BTreeMap::new();

    for (_index, substring) in substrings.iter().enumerate() {
        for found in input.match_indices(substring) {
            let absolute_index = found.0;
            map.insert(absolute_index, substring.to_string());
        }
    }
    // if map.is_empty() {
    //     println!("No substrings found for {}", input);
    // } else {
    //     println!("Substrings found at map: {:?} from {}", map, input);
    // }

    map
}

fn is_numeric(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

pub fn find_and_replace_substrings(input: &str) -> String {
    let word_to_number_map = create_word_to_number_map();
    let mut keys: Vec<&str> = word_to_number_map.keys().map(|s| s.as_str()).collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len()));

    let map = find_all_substrings_in_array(input, &keys);

    let mut modified_input = String::from(input);
    let mut last_pos: &usize = &0;
    let mut last_value = "";
    let mut index_offset = 0;

    // Iterate over the BTreeMap
    // search from 2 ends, if found 2 ends, then stop searching
    for (pos, value) in map.iter() {
        if is_numeric(value) {
            break;
        }
        if let Some(&number) = word_to_number_map.get(&value.to_lowercase()) {
            modified_input.replace_range(
                (pos - index_offset)..(value.len() + pos - index_offset),
                &number.to_string(),
            );
            index_offset += &value.len() - &number.to_string().len();
            last_pos = pos;
            last_value = value;
            break;
        }
    }

    if map.len() > 1 {
        for (pos, value) in map.iter().rev() {
            if is_numeric(value) {
                break;
            }
            if let Some(&number) = word_to_number_map.get(&value.to_lowercase()) {
                modified_input.replace_range(
                    (pos - index_offset)..(value.len() + pos - index_offset),
                    &number.to_string(),
                );
                break;
            }
        }
    }

    // old method of replacing strings from left to right, not used anymore
    // for (pos, value) in map.iter() {
    //     if pos - last_pos < last_value.len() {
    //         continue;
    //     }
    //     if let Some(&number) = word_to_number_map.get(&value.to_lowercase()) {
    //         modified_input.replace_range(
    //             (pos - index_offset)..(value.len() + pos - index_offset),
    //             &number.to_string(),
    //         );
    //         index_offset += &value.len() - &number.to_string().len();
    //         last_pos = pos;
    //         last_value = value;
    //     }
    // }
    // println!("{:?}", modified_input);

    modified_input
}
