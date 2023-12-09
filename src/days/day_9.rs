use std::fs::File;
use std::io::{self, BufRead};

pub fn process_file(file_path: &str) -> i32 {
    let mut sums: Vec<i32> = Vec::new();

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
                println!("{:?}", values);
                sums.push(find_next_history_value(&values));
            }
        }

        println!("sums: {}", sums.iter().fold(0, |acc, &x| acc + x));
    } else {
        eprintln!("Error opening file: {}", file_path);
    }

    sums.iter().fold(0, |acc, &x| acc + x)
}

fn find_next_history_value(values: &Vec<i32>) -> i32 {
    let mut original_vector = values.clone();
    let mut offsets: Vec<i32> = Vec::new();
    let Some(&last_element) = original_vector.last().clone() else {
        panic!("can't get last element");
    };
    offsets.push(last_element);

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
            println!("offsets {:?}", &offsets);
            break;
        }

        println!("{:?}", &next_vector);
        let Some(&last_element) = next_vector.last().clone() else {
            panic!("can't get last element");
        };
        offsets.push(last_element);
        original_vector = next_vector;
    }

    offsets.iter().fold(0, |acc, &x| acc + x)
}
