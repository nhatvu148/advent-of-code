use std::fs::File;
use std::io::{self, BufRead};

pub fn process_file(file_path: &str) -> (i32, i32) {
    let mut start_sums: Vec<i32> = Vec::new();
    let mut end_sums: Vec<i32> = Vec::new();

    // Open the file and create a BufReader to efficiently read lines
    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        // Iterate over each line in the file
        for line in reader.lines() {
            if let Ok(line_content) = line {
                let values: Vec<i32> = line_content
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                let (start, end) = find_next_history_value(&values);
                start_sums.push(start);
                end_sums.push(end);
            }
        }
    } else {
        eprintln!("Error opening file: {}", file_path);
    }

    (
        start_sums.iter().fold(0, |acc, &x| acc + x),
        end_sums.iter().fold(0, |acc, &x| acc + x),
    )
}

fn find_next_history_value(values: &Vec<i32>) -> (i32, i32) {
    let mut original_vector = values.clone();
    let mut start_offsets: Vec<i32> = Vec::new();
    let mut end_offsets: Vec<i32> = Vec::new();

    let (first_element, last_element) = get_first_and_last(&original_vector);
    start_offsets.push(first_element);
    end_offsets.push(last_element);

    loop {
        let next_vector =
            original_vector
                .windows(2)
                .fold(Vec::new(), |mut acc, window| {
                    let difference = window[1] - window[0];
                    acc.push(difference);
                    acc
                });

        // Check if the next_vector contains all zeros
        if next_vector.iter().all(|&diff| diff == 0) {
            break;
        }

        let (first_element, last_element) = get_first_and_last(&next_vector);
        start_offsets.push(first_element);
        end_offsets.push(last_element);
        original_vector = next_vector;
    }

    (
        start_offsets.iter().rev().fold(0, |acc, &x| x - acc),
        end_offsets.iter().fold(0, |acc, &x| acc + x),
    )
}

fn get_first_and_last<T: Clone>(vector: &Vec<T>) -> (T, T) {
    let Some(first_element) = vector.first().cloned() else {
        panic!("can't get first element");
    };
    let Some(last_element) = vector.last().cloned() else {
        panic!("can't get last element");
    };

    (first_element, last_element)
}
